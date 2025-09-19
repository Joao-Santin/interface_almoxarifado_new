use iced::widget::shader::wgpu::util::align_to;
use iced::widget::{button, text, text_input, row, column, keyed_column, radio, scrollable};
use iced::{Alignment::{Center, Start, End}};
use iced::Length::{Fill, Fixed};
use iced::{Element, Task as Command};// Subscription para caso precise iniciar algo assim que rodar.
use egestorapi_test::{AjusteEstoque, AppLogic, Estoque, ItemRetirada, TipoMovimento};

#[derive(Debug, Clone)]
enum Message{
    GetAppLogic,
    GotAppLogic(Result<AppLogic, String>),
    InputChanged(CamposInput, String),
    Changescreen(Screens),
    TrocouTipoMovimento(TipoMovimento),
    AdicionarAoCarrinho(ItemRetirada)
}

#[derive(Clone, Debug)]
enum CamposInput{
    Filtro,
    QtdMovimento
}
#[derive(Debug, Clone)]
enum Screens{
    Main,// tela principal, seleção de itens.
    Carrinho,// tela para checagem do que vai retirar.
    Contador(Estoque),// tela que vai adicionar o item que quer retirar.
}

struct AlmoxarifadoApp{
    app_logic: Option<AppLogic>,
    token: String,
    filter: String,
    qtd_movimento: f32,
    qtd_movimento_txt: String,
    tipo_movimento: TipoMovimento,
    estoque: AjusteEstoque,
    screen: Screens
    
}

impl Default for AlmoxarifadoApp{
    fn default() -> Self {
        Self{
            app_logic: None,
            token: String::new(),
            filter: String::new(),
            qtd_movimento: 0.0,
            qtd_movimento_txt: String::new(),
            tipo_movimento: TipoMovimento::Retirada,
            estoque: AjusteEstoque::new(),
            screen: Screens::Main
        }
    }
}

impl AlmoxarifadoApp{
    async fn init_app_logic()-> Result<AppLogic, String>{
        AppLogic::new().await.map_err(|e| e.to_string())
    }

    fn update(&mut self, message: Message)-> Command<Message>{
        match message{
            Message::GetAppLogic => {
                println!("Getting App logic");
                Command::perform(Self::init_app_logic(), Message::GotAppLogic)
            },
            Message::GotAppLogic(Ok(app_logic_got)) => {
                self.app_logic = Some(app_logic_got);
                if let Some(app_logic) = &mut self.app_logic {
                    app_logic.ajuste_estoque.get_estoque(app_logic.relatorios.estoques.clone());
                    println!("Teste token: {}", &app_logic.token.access_token);
                    let estoque: &Vec<Estoque> = &app_logic.ajuste_estoque.estoque;
                    for item in estoque{
                        println!("{}", item.produto)
                    }
                }
                Command::none()
            }, 
            Message::GotAppLogic(Err(erro)) => {
                println!("erro:{}", erro);
                Command::none()
            }
            Message::InputChanged(campo, palavra) => {
                match campo{
                    CamposInput::Filtro => {
                        self.filter = palavra;
                        Command::none()
                    },
                    CamposInput::QtdMovimento => {
                        let palavra = if palavra.is_empty() {"0".to_string()} else { palavra };

                        if palavra.chars().all(|c| c.is_ascii_digit() || c == '.') {
                            if let Ok(valor) = palavra.parse::<f32>() {
                                self.qtd_movimento = valor;
                                self.qtd_movimento_txt = self.qtd_movimento.clone().to_string()
                            }
                        }
                        Command::none()
                    }
                }
            }

            Message::Changescreen(screen_vindo) =>{
                self.screen = screen_vindo;
                Command::none()
            }
            Message::TrocouTipoMovimento(tipo_movimento) =>{
                self.tipo_movimento = tipo_movimento;
                Command::none()
            }
            Message::AdicionarAoCarrinho(item_retirada)=>{
                if let Some(app_logic) = &mut self.app_logic{
                    app_logic.ajuste_estoque.add_item_carrinho(item_retirada);
                    println!("-Itens Adicionados-");
                    for item in app_logic.ajuste_estoque.carrinhoretirada.iter(){
                        println!("{}", item.produto)
                    }
                    return self.update(Message::Changescreen(Screens::Main));
                }else{
                    println!("Falta App logic aqui...")
                }
                Command::none()

            }
        }
    }
    fn view(&self) -> Element<Message> {
        match &self.screen{
            Screens::Main =>{
                let title = text("almoxarifado")
                    .width(Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5])
                    .align_x(Center);
                let quant_itens_carrinho: i32;
                if let Some(app_logic) = &self.app_logic{
                    quant_itens_carrinho = app_logic.ajuste_estoque.carrinhoretirada.len() as i32
                }else{
                    quant_itens_carrinho = 0
                }
                let button_row = row![
                    button(text(format!("Cart: {}", quant_itens_carrinho.to_string())))
                        .padding([10, 5])
                        .on_press(Message::Changescreen(Screens::Carrinho)),
                    button(text("Refresh"))
                        .padding([10, 5])
                        .on_press(Message::GetAppLogic),
                ];
                let input_filter = text_input("O que precisa para hoje?", &self.filter)
                    .size(35)
                    .width(1500)
                    .align_x(Center)    
                    .on_input(|value| Message::InputChanged(CamposInput::Filtro, value));
                let estoque_vazio: Vec<Estoque> = Vec::new();
                let cabecalho = row![
                    column![
                        text("CODIGO").size(20),
                    ].width(Fixed(150.0)).align_x(Center),
                    column![
                        text("DESCRICAO").size(20),
                    ].width(Fixed(200.0)).align_x(Start),
                    column![
                        text("ESTOQUE").size(20),
                    ].width(Fixed(150.0)).align_x(Center),
                ].spacing(15);
                let itens = keyed_column(
                        {
                            let estoque = if let Some(app_logic) = &self.app_logic {
                                &app_logic.ajuste_estoque.estoque
                            } else {
                                &estoque_vazio
                            };
                            estoque
                                .iter()
                                .enumerate()
                                .filter(|(_, item)| {
                                    if self.filter.trim() == "" {
                                        true
                                    } else {
                                        item.produto.to_lowercase().contains(&self.filter.to_lowercase())
                                    }
                                })
                                .map(|(_, item)| {
                                    (
                                        item.codigo,
                                        row![
                                            column![
                                                text(item.codigo)
                                            ].width(Fixed(150.0)).align_x(Center),
                                            column![
                                                text(format!("{}", item.produto))
                                            ].width(Fixed(200.0)).align_x(Start),
                                            column![
                                                text(item.estoque)
                                            ].width(Fixed(150.0)).align_x(Center),
                                            column![
                                                button(text("<retirar>")).on_press(Message::Changescreen(Screens::Contador(Estoque{
                                                    codigo: item.codigo,
                                                    produto: item.produto.clone(),
                                                    estoque: item.estoque,
                                                    custo: item.custo.clone(),
                                                    total: item.total
                                                })))
                                            ].width(Fixed(100.0)).align_x(Center)
                                        ].spacing(15).into()
                                    )
                                })
                        }
                    ).spacing(15);
                column![
                    title, button_row, input_filter,cabecalho, scrollable(column![itens].spacing(15)).height(Fill).width(Fill)
                ].into()
            }
            Screens::Carrinho => {
                row![
                    column![
                        row![text("text_input"),button(text("testando"))],
                        column![text("pick_list"),],
                    ],
                    column![
                        text("Codigo:"),
                        text("Nome:"),
                        text("Estoque:"),
                        text("Quantidade Mov:"),
                        row![
                            text("-retirada-"),
                            text("-entrada-")
                        ],

                    ],
                ].into()
            }
            Screens::Contador(estoque) => {
                row![
                    column![
                        text("adicionar ao carrinho")
                            .width(Fill)
                            .size(60)
                            .color([0.5, 0.5, 0.5])
                            .align_x(Center),
                        row![
                            column![
                                row![
                                    text(format!("Codigo:")).color([0.5, 0.5, 0.5]).size(20),
                                ],
                                row![
                                    text(format!("Descricao:")).color([0.5, 0.5, 0.5]).size(20),
                                ].height(Fixed(125.0)).align_y(Center),
                                row![
                                    text(format!("Estoque:")).color([0.5, 0.5, 0.5]).size(20),
                                ],
                                row![
                                    text(format!("Movimentar:")).color([0.5, 0.5, 0.5]).size(20),
                                ],
                                row![
                                    text(format!("Tipo:")).color([0.5, 0.5, 0.5]).size(20),
                                ],
                                row![
                                    button(text("Voltar").size(30)).width(100).height(50).on_press(Message::Changescreen(Screens::Main)),
                                ],
                            ].align_x(Center).width(Fixed(300.0)).spacing(20),
                            column![
                                row![
                                    text(&estoque.codigo).size(20),
                                ],
                                row![
                                    text(&estoque.produto).size(20),
                                ].height(Fixed(125.0)).align_y(Center),
                                row![
                                    text(&estoque.estoque).size(20),
                                ],
                                row![
                                    text_input("Quantos movimentar?", &self.qtd_movimento_txt)
                                        .on_input(|value| Message::InputChanged(CamposInput::QtdMovimento, value))
                                        .width(300)
                                        .size(20)
                                        .align_x(Center),
                                ],
                                row![
                                    radio("Entrada", TipoMovimento::Entrada, Some(self.tipo_movimento), Message::TrocouTipoMovimento).size(20),
                                    radio("Retirada", TipoMovimento::Retirada, Some(self.tipo_movimento), Message::TrocouTipoMovimento).size(20),
                                ].spacing(40),
                                row![
                                    button(text("Add").size(30)).width(100).height(50).on_press(Message::AdicionarAoCarrinho(ItemRetirada {
                                        codigo: estoque.codigo,
                                        produto: estoque.produto.clone(),
                                        tipo: self.tipo_movimento,
                                        quantidade: self.qtd_movimento,
                                        estoqueatual: estoque.estoque
                                    }))
                                ]
                            ].align_x(Start).width(Fixed(300.0)).spacing(20),
                        ].spacing(15),
                    ].align_x(Center).spacing(50)
                ].align_y(Center).into()
            }
        }
    }
}

fn main() -> iced::Result{
    dotenv::dotenv().ok();
    iced::application("Almoxarifado Biplas", AlmoxarifadoApp::update, AlmoxarifadoApp::view)
    .window_size((800.0, 800.0))
    .run()
}
