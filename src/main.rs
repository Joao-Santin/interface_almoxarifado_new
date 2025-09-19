use iced::widget::{button, text, text_input, row, column, keyed_column, radio};
use iced::Alignment::Center;
use iced::Length::Fill;
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
                let button_row = row![
                    button(text(format!("carrinho: {}", "5")))
                    .on_press(Message::Changescreen(Screens::Carrinho)),
                    button(text("historico")).on_press(Message::GetAppLogic),
                    button(text("categoria")),

                ];
                let input_filter = text_input("O que precisa para hoje?", &self.filter)
                        .on_input(|value| Message::InputChanged(CamposInput::Filtro, value));
                let estoque_vazio: Vec<Estoque> = Vec::new();
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
                                            text(format!("{} - {} - {}", item.codigo, item.produto, item.estoque)),
                                            button(text("<retirar>")).on_press(Message::Changescreen(Screens::Contador(Estoque{
                                                codigo: item.codigo,
                                                produto: item.produto.clone(),
                                                estoque: item.estoque,
                                                custo: item.custo.clone(),
                                                total: item.total
                                            })))
                                        ].into()
                                    )
                                })
                        }
                    ).spacing(10);

                column![
                    title, button_row, input_filter, itens
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
                        text("Adicionador carrinho"),
                        text(format!("codigo: {}", &estoque.codigo)),
                        text(format!("item: {}", &estoque.produto)),
                        text(format!("estoque: {}", &estoque.estoque)),
                        text_input("Quantos movimentar?", &self.qtd_movimento_txt).on_input(|value| Message::InputChanged(CamposInput::QtdMovimento, value)),
                        row![
                            text("Tipo:"),
                            radio("Entrada", TipoMovimento::Entrada, Some(self.tipo_movimento), Message::TrocouTipoMovimento),
                            radio("Retirada", TipoMovimento::Retirada, Some(self.tipo_movimento), Message::TrocouTipoMovimento),
                            row![
                                button(text("Voltar")).on_press(Message::Changescreen(Screens::Main)),
                                button(text("ADD Carr.")).on_press(Message::AdicionarAoCarrinho(ItemRetirada {
                                    codigo: estoque.codigo,
                                    produto: estoque.produto.clone(),
                                    tipo: self.tipo_movimento,
                                    quantidade: self.qtd_movimento,
                                    estoqueatual: estoque.estoque
                                }))
                            ]
                        ]
                    ]
                ].into()
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
