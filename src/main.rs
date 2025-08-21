use iced::widget::{button, text, text_input, row, column, center};
use iced::{Element, Task as Command};// Subscription para caso precise iniciar algo assim que rodar.
use egestorapi_test::{ERPToken, AjusteEstoque};
#[derive(Debug, Clone)]
enum Message{
    Increment,
    Gettoken,
    Gottoken(Result<String, String>),
    Changescreen(Screens)
}

#[derive(Debug, Clone)]
enum Screens{
    Main,
    Carrinho
}

struct AlmoxarifadoApp{
    token: String,
    counter: Counter,
    screen: Screens
    
}

impl Default for AlmoxarifadoApp{
    fn default() -> Self {
        Self{
            token: String::new(),
            counter: Counter::default(),
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
            Message::Increment => {
                self.counter.value += 1;
                Command::none()
            },
            Message::Gettoken => {
                Command::perform(Self::get_token(), Message::Gottoken)
            },
            Message::Gottoken(Ok(token_got)) => {
                self.token = token_got;
                Command::none()
            }
            Message::Gottoken(Err(erro)) => {
                println!("erro: {}", erro);
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
                button(text(self.counter.value)).on_press(Message::Changescreen(Screens::Carrinho)).into()
            }
            Screens::Carrinho => {
                text("teste apenas").into()
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
