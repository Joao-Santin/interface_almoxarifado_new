#[cfg(windows)]
use iced::widget::{button, column, keyed_column, radio, row, scrollable, text, text_input, Space, container};
use iced::{Alignment::{Center, Start}};
use iced::Length::{self, Fill, Fixed};
use iced::{Element, Task as Command};// Subscription para caso precise iniciar algo assim que rodar.
use egestorapi_test::{AjusteEstoque, AppLogic, Estoque, ItemRetirada, TipoMovimento, ItemResumo};

#[derive(Debug, Clone)]
enum Message{
    GetAppLogic,
    GotAppLogic(Result<AppLogic, String>),
    InputChanged(CamposInput, String),
    Changescreen(Screens),
    TrocouTipoMovimento(TipoMovimento),
    AdicionarAoCarrinho(ItemRetirada),
    RetirouDoCarrinho(u32),
    Resumir,
    GetAjustarEstoque,
    GotAjustarEstoque(bool),
    GotAppLogicThenChangeScreen(AppLogic),
    ChangeFilterCarrinho,
    SliceCodigoBarrasTxt,
}

#[derive(Clone, Debug)]
enum CamposInput{
    CodigoBarrasTxt,
    Filtro,
    QtdMovimento
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum FiltroTipoMovimento{
    Geral,
    Retirada,
    Entrada
}

impl FiltroTipoMovimento{
    fn to_text(&self) -> String{
        match self{
            FiltroTipoMovimento::Geral => "Geral".to_string(),
            FiltroTipoMovimento::Retirada => "Retirada".to_string(),
            FiltroTipoMovimento::Entrada => "Entrada".to_string(),
        }

    }

}

#[derive(Debug, Clone)]
enum Screens{
    Main,// tela principal, seleção de itens.
    Carrinho,// tela para checagem do que vai retirar.
    Contador(Estoque),// tela que vai adicionar o item que quer retirar.
    Resumidor(Vec<ItemResumo>)//produto, item(codigo e estoque final),
    //estoque atual e quantidade retirada
}

struct AlmoxarifadoApp{
    app_logic: Option<AppLogic>,
    token: String,
    filter: String,
    codigo_de_barras_txt: String,
    filtro_tipo_selecionado: FiltroTipoMovimento,
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
            codigo_de_barras_txt: String::new(),
            filtro_tipo_selecionado: FiltroTipoMovimento::Geral,
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
                Command::perform(Self::init_app_logic(), Message::GotAppLogic)
            },
            Message::GotAppLogic(Ok(app_logic_got)) => {
                self.app_logic = Some(app_logic_got);
                if let Some(app_logic) = &mut self.app_logic {
                    app_logic.ajuste_estoque.get_estoque(app_logic.relatorios.estoques.clone());
                }
                Command::none()
            }, 
            Message::GotAppLogic(Err(erro)) => {
                println!("erro:{}", erro);
                Command::none()
            }
            Message::InputChanged(campo, palavra) => {
                match campo{
                    CamposInput::CodigoBarrasTxt => {
                        self.codigo_de_barras_txt = palavra;
                        Command::none()
                    }
                    CamposInput::Filtro => {
                        self.filter = palavra;
                        Command::none()
                    },
                    CamposInput::QtdMovimento => {
                        if palavra.is_empty(){
                            self.qtd_movimento = 0.0;
                            self.qtd_movimento_txt = String::new();
                            return Command::none();
                        }

                        if palavra.chars().all(|c| c.is_ascii_digit() || c == '.') {
                            if let Ok(valor) = palavra.parse::<f32>() {
                                self.qtd_movimento = valor;
                                self.qtd_movimento_txt = palavra;
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
                    return self.update(Message::Changescreen(Screens::Main));
                }else{
                    println!("Falta App logic aqui...")
                }
                Command::none()

            }
            Message::RetirouDoCarrinho(codigo) =>{
                if let Some(app_logic) = &mut self.app_logic{
                    app_logic.ajuste_estoque.del_item_carrinho(codigo)
                }else{
                    println!("Falta App logic aqui...")
                }
                Command::none()
            }
            Message::Resumir => {
                if let Some(app_logic) = &mut self.app_logic{
                    app_logic.ajuste_estoque.resumir(app_logic.relatorios.estoques_geral.clone());
                    let resumo = app_logic.ajuste_estoque.resumoretirada.clone();
                    drop(app_logic);

                    self.update(Message::Changescreen(Screens::Resumidor(resumo)));
                Command::none()
                }else{
                    println!("Falta App logic aqui...");
                    Command::none()
                }
            }
            Message::GetAjustarEstoque => {
                if let Some(app_logic) = &mut self.app_logic{
                    let client = app_logic.client.clone();
                    let token = app_logic.token.clone();
                    let ajuste = app_logic.ajuste_estoque.clone();
                    return Command::perform(
                        async move {
                            ajuste.realizar_operacao(client, token).await
                        },
                        |resultado| Message::GotAjustarEstoque(resultado)
                    )
                }else{
                    println!("Falta App logic aqui...");
                    Command::none()
                }
            }
            Message::GotAjustarEstoque(boleano) =>{
                match boleano {
                    true =>{
                        if let Some(app_logic) = &mut self.app_logic{
                            return Command::perform(
                                Self::init_app_logic(),
                                |res| match res {
                                    Ok(app_logic_got) => Message::GotAppLogicThenChangeScreen(app_logic_got),
                                    Err(err) => Message::GotAppLogic(Err(err)),
                                },
                            );
                        }
                        Command::none()
                    },
                    false => self.update(Message::Changescreen(Screens::Carrinho))
                };
                Command::none()
            }
            Message::GotAppLogicThenChangeScreen(app_logic_got) => {
                self.app_logic = Some(app_logic_got);
                if let Some(app_logic) = &mut self.app_logic{
                    app_logic.ajuste_estoque.get_estoque(app_logic.relatorios.estoques.clone());
                }
                self.update(Message::Changescreen(Screens::Main))
            }
            Message::ChangeFilterCarrinho => {
                match self.filtro_tipo_selecionado {
                    FiltroTipoMovimento::Geral => {
                        self.filtro_tipo_selecionado = FiltroTipoMovimento::Entrada;
                    },
                    FiltroTipoMovimento::Entrada => {
                        self.filtro_tipo_selecionado = FiltroTipoMovimento::Retirada;
                    }
                    FiltroTipoMovimento::Retirada => {
                        self.filtro_tipo_selecionado = FiltroTipoMovimento::Geral;
                    }
                }
                Command::none()

            }
            Message::SliceCodigoBarrasTxt => {
                if let Some((codigo, quantidade)) = &self.codigo_de_barras_txt.trim().split_once("-"){
                    if let (Ok(valor), Ok(valor2)) = (codigo.trim().parse::<u32>(), quantidade.trim().parse::<f32>()){
                        let codigo = valor;
                        let quantidade = valor2;
                        ////continar aqui!
                        if let Some(app_logic) = &mut self.app_logic{
                            let item = app_logic.ajuste_estoque.get_itemretirada_by_id_e_quant(codigo, quantidade, TipoMovimento::Retirada).unwrap();
                            app_logic.ajuste_estoque.add_item_carrinho(item);
                        }
                        self.codigo_de_barras_txt = String::new();
                        Command::none()
                    }else{
                        self.codigo_de_barras_txt = String::new();
                        Command::none()
                    }
                }else{
                    self.codigo_de_barras_txt = String::new();
                    Command::none()
                    
                }

            }
        }
    }
    fn view(&self) -> Element<Message> {
        match &self.screen{
            Screens::Main =>{
                let title = text("almoxarifado")
                    .width(Fill)
                    .size(60)
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
                    button(text("Carregar!"))
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
                    ].width(Fixed(150.0)).align_x(Start),
                    column![
                        text("DESCRICAO").size(20),
                    ].width(Fixed(200.0)).align_x(Start),
                    column![
                        text("ESTOQUE").size(20),
                    ].width(Fixed(150.0)).align_x(Start),
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
                    title, button_row, input_filter,cabecalho, scrollable(container(column![itens].spacing(15)).align_x(Center).width(Fill)).height(Fill)
                ].align_x(Center).into()
            }
            Screens::Carrinho => {
                fn comparacao_match(tipinho_item: TipoMovimento, tipinho_filtro: FiltroTipoMovimento) -> bool{
                    match tipinho_item{
                        TipoMovimento::Entrada => {
                            if tipinho_filtro == FiltroTipoMovimento::Entrada{
                                true
                            }else{
                                false
                            }
                        },
                        TipoMovimento::Retirada => {
                            if tipinho_filtro == FiltroTipoMovimento::Retirada{
                                true
                            }else{
                                false
                            }

                        }
                    }
                }

                let itens_retirada_vazio: Vec<ItemRetirada> = Vec::new();
                let itens = keyed_column(
                    {
                        let itens_retirada = if let Some(app_logic) = &self.app_logic{
                            &app_logic.ajuste_estoque.carrinhoretirada
                        } else {
                            &itens_retirada_vazio
                        };
                        itens_retirada
                            .iter()
                            .enumerate()
                            .filter(|(_, item)| {
                                if self.filtro_tipo_selecionado == FiltroTipoMovimento::Geral{
                                    true
                                }else{
                                    comparacao_match(item.tipo, self.filtro_tipo_selecionado)
                                }
                            })
                            .map(|(_, item)| {
                                (
                                    item.codigo,
                                    row![
                                        column![
                                            text(item.codigo)
                                        ].width(Fixed(100.0)).align_x(Center),
                                        column![
                                            text(format!("{}", item.produto))
                                        ].width(Fixed(250.0)).align_x(Start),
                                        column![
                                            text(format!("{}", item.tipo))
                                        ].width(Fixed(100.0)).align_x(Center),
                                        column![
                                            text(format!("{}", item.estoqueatual))
                                        ].width(Fixed(150.0)).align_x(Center),
                                        column![
                                            text(format!("{}", item.quantidade))
                                        ].width(Fixed(120.0)).align_x(Center),
                                        column![
                                            button(text("apagar")).on_press(Message::RetirouDoCarrinho(item.codigo))
                                        ]
                                    ].into()
                                )
                            })
                    }
                ).spacing(15);
                column![
                    column![
                        text("carrinho")
                            .width(Fill)
                            .size(60)
                            .color([0.5, 0.5, 0.5])
                            .align_x(Center),
                        row![
                            text("Filtro:"),
                            Space::with_width(Length::Fixed(20.0)),
                            button(text(self.filtro_tipo_selecionado.to_text())).on_press(Message::ChangeFilterCarrinho),
                            Space::with_width(Length::Fixed(20.0)),
                            text_input("Codigo de Barras", &self.codigo_de_barras_txt).on_input(|value| Message::InputChanged(CamposInput::CodigoBarrasTxt, value)).on_submit(Message::SliceCodigoBarrasTxt).width(Fixed(180.0)),
                        ],
                        row![
                            column![
                                text("codigo").size(20).color([0.5, 0.5, 0.5])
                            ].width(Fixed(100.0)).align_x(Start),
                            column![
                                text("descricao").size(20).color([0.5, 0.5, 0.5])
                            ].width(Fixed(250.0)).align_x(Start),
                            column![
                                text("tipo").size(20).color([0.5, 0.5, 0.5])
                            ].width(Fixed(100.0)).align_x(Start),
                            column![
                                text("estoque atual").size(20).color([0.5, 0.5, 0.5])
                            ].width(Fixed(150.0)).align_x(Center),
                            column![
                                text("quantidade").size(20).color([0.5, 0.5, 0.5])
                            ].width(Fixed(120.0)).align_x(Center)
                        ],
                        scrollable(container(column![itens]).align_x(Center).width(Fill)).height(Fill),
                        row![
                            button(text("Voltar").size(30)).width(100).height(50).on_press(Message::Changescreen(Screens::Main)),
                        button(text("<OPERACAO>").size(30)).width(220).height(50).on_press(Message::Resumir)
                        ].spacing(15),
                    ].spacing(15).align_x(Center),
                ].align_x(Center).into()
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
            },
            Screens::Resumidor(itens_resumo) => {
                let itens = keyed_column(itens_resumo.iter().map(|item| {
                (
                    item.codproduto.clone(),
                    row![
                        column![
                            text(item.codproduto)
                        ].width(Fixed(150.0)).align_x(Center),
                        column![
                            text(item.estoquefinal)
                        ].width(Fixed(150.0)).align_x(Center),
                    ].into()
                    )
                }));
                row![
                    column![
                        text("conferencia").width(Fill).size(60).color([0.5, 0.5, 0.5]).align_x(Center),
                        text("area reservada para identificacao de erros caso tenha. se nao responsavel, apenas clicar em concluir!").width(Fill).size(15).color([0.5, 0.5, 0.5]).align_x(Center),
                        row![
                            column![
                                text("CODIGO").size(20).color([0.5, 0.5, 0.5])
                            ].width(Fixed(150.0)).align_x(Center),
                            column![
                                text("ESTOQUE FINAL").size(20).color([0.5, 0.5, 0.5])
                            ].width(Fixed(150.0)).align_x(Center),
                        ].spacing(15),
                        scrollable(container(column![itens]).align_x(Center).width(Fill)).height(Fill),
                        row![
                        button(text("Voltar").size(30)).width(100).height(50).on_press(Message::Changescreen(Screens::Carrinho)),
                        button(text("Finalizar").size(30).center()).width(220).height(50).on_press(Message::GetAjustarEstoque)
                        ].spacing(15)
                    ].align_x(Center).spacing(15),

                ].into()
            }
        }
    }
}

fn main() -> iced::Result{
    dotenv::dotenv().ok();
    iced::application("Almoxarifado Biplas", AlmoxarifadoApp::update, AlmoxarifadoApp::view)
        .window_size((800.0, 600.0))
        .run()
}
