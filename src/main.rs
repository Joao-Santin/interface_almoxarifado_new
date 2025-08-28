use iced::widget::{button, text, text_input, row, column, center, keyed_column};
use iced::Alignment::Center;
use iced::Length::Fill;
use iced::{Element, Task as Command};// Subscription para caso precise iniciar algo assim que rodar.
use egestorapi_test::{ERPToken, AjusteEstoque, AppLogic};

#[derive(Debug, Clone)]
enum Message{
    Gettoken,
    Gottoken(Result<String, String>),
    GetAppLogic,
    GotAppLogic(Result<AppLogic, String>),
    InputChanged(CamposInput, String),
    Filter,
    Changescreen(Screens)
}

#[derive(Clone, Debug)]
enum CamposInput{
    Filtro,
}
#[derive(Debug, Clone)]
enum Screens{
    Main,
    Carrinho
}

struct AlmoxarifadoApp{
    app_logic: Option<AppLogic>,
    token: String,
    filter: String,
    estoque: AjusteEstoque,
    screen: Screens
    
}

impl Default for AlmoxarifadoApp{
    fn default() -> Self {
        Self{
            app_logic: None,
            token: String::new(),
            filter: String::new(),
            estoque: AjusteEstoque::new(),
            screen: Screens::Main
        }
    }
}

impl AlmoxarifadoApp{
    async fn get_token()->Result<String, String>{
        ERPToken::get_access_token().await
    }
    async fn init_app_logic()-> Result<AppLogic, String>{
        AppLogic::new().await.map_err(|e| e.to_string())
    }

    fn update(&mut self, message: Message)-> Command<Message>{
        match message{
            Message::Gettoken => {
                Command::perform(Self::get_token(), Message::Gottoken)
            },
            Message::Gottoken(Ok(token_got)) => {
                self.token = token_got;
                Command::none()
            },
            Message::Gottoken(Err(erro)) => {
                println!("erro: {}", erro);
                Command::none()
            },
            Message::GetAppLogic => {
                println!("Getting App logic");
                Command::perform(Self::init_app_logic(), Message::GotAppLogic)
            },
            Message::GotAppLogic(Ok(app_logic_got)) => {
                self.app_logic = Some(app_logic_got);
                if let Some(app_logic) = &self.app_logic {
                    println!("Teste token: {}", app_logic.token.access_token);
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
                    }
                }
            }
            Message::Filter => {
                //self.estoque.get_estoque() //fazer na lib um metodo para coleta de estoque de forma
                //mais rapida
                println!("filtrando por:{}", self.filter);
                Command::none()
            }

            Message::Changescreen(screen_vindo) =>{
                self.screen = screen_vindo;
                Command::none()
            }
        }
    }
    fn view(&self) -> Element<Message> {
        match self.screen{
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
                        .on_input(|value| Message::InputChanged(CamposInput::Filtro, value))
                        .on_submit(Message::Filter);
                let itens = keyed_column(
                    self.estoque.estoque
                        .iter()
                        .enumerate()
                        .filter(|(_, item)| {
                            if self.filter.is_empty() {
                                true
                            } else {
                                item.produto.to_lowercase().contains(&self.filter.to_lowercase())
                            }
                        })
                        .map(|(i, item)| {
                            (
                                i,
                                text(format!("{} - {}", item.codigo, item.produto)).into()
                            )
                        })
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
