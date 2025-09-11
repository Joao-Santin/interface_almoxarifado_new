use iced::widget::{button, text, text_input, row, column, keyed_column, radio};
use iced::Alignment::Center;
use iced::Length::Fill;
use iced::{Element, Task as Command};// Subscription para caso precise iniciar algo assim que rodar.
use egestorapi_test::{AjusteEstoque, AppLogic, ERPToken, Estoque};

#[derive(Debug, Clone)]
enum Message{
    GetAppLogic,
    GotAppLogic(Result<AppLogic, String>),
    InputChanged(CamposInput, String),
    Changescreen(Screens)
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
    qtd_movimento: String,
    estoque: AjusteEstoque,
    screen: Screens
    
}

impl Default for AlmoxarifadoApp{
    fn default() -> Self {
        Self{
            app_logic: None,
            token: String::new(),
            filter: String::new(),
            qtd_movimento: String::new(),
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
                        self.qtd_movimento = palavra;
                        Command::none()
                    }
                }
            }

            Message::Changescreen(screen_vindo) =>{
                self.screen = screen_vindo;
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
                    button(text("carrinho"))
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
                        text_input("Quantos movimentar?", &self.qtd_movimento),
                        row![
                            text("Tipo:"),
                            radio("Entrada", )//continuar aqui, não sei como funciona radio buttons
                        ]
                    ]
                ].into()
            }
        }
    }

}

#[derive(Default)]
struct Counter{
    value:u64,
}

fn main() -> iced::Result{
    dotenv::dotenv().ok();
    iced::application("Almoxarifado Biplas", AlmoxarifadoApp::update, AlmoxarifadoApp::view)
    .window_size((800.0, 800.0))
    .run()
}
