use iced::widget::{button, text, text_input, row, column, center, keyed_column};
use iced::{Element, Task as Command};// Subscription para caso precise iniciar algo assim que rodar.
use egestorapi_test::{ERPToken, AjusteEstoque, ItemRetirada};

#[derive(Debug, Clone)]
enum Message{
    Gettoken,
    Gottoken(Result<String, String>),
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
    token: String,
    filter: String,
    itens: Vec<ItemRetirada>,
    screen: Screens
    
}

impl Default for AlmoxarifadoApp{
    fn default() -> Self {
        Self{
            token: String::new(),
            filter: String::new(),
            itens: Vec::new(),
            screen: Screens::Main
        }
    }
}

impl AlmoxarifadoApp{
    async fn get_token()->Result<String, String>{
        ERPToken::get_access_token().await
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
            Message::InputChanged(campo, palavra) =>{
                match campo{
                    CamposInput::Filtro => {
                        self.filter = palavra;
                        Command::none()
                    }
                }
            }
            Message::Filter => {
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
                column![
                    text("ALMOXARIFADO"),
                    row![
                        button(text("carrinho"))
                        .on_press(Message::Changescreen(Screens::Carrinho)),
                        button(text("historico")),
                        button(text("categoria")),
                    ],
                    text_input("O que precisa para hoje?", &self.filter)
                        .on_input(|value| Message::InputChanged(CamposInput::Filtro, value))
                        .on_submit(Message::Filter),
                    keyed_column(
                        self.itens.iter().map(|i| )//continuar aqui, usar exemplo de todos. precisa
                        //criar uma forma de buscar os itens e filtrar o que faz sentido para ser
                        //displayado. Basicamente e isso. Esta na linha 216 do todos.
                    )
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
    iced::application("Almoxarifado Biplas", AlmoxarifadoApp::update, AlmoxarifadoApp::view)
    .run()
}
