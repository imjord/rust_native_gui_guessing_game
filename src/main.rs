extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use rand::Rng;






#[derive(Default, NwgUi)]
pub struct GuessingApp {
    #[nwg_control(size: (500, 120), position: (300, 300), title: "Guessing Game", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnKeyEnter: [GuessingApp::guess], OnWindowClose: [GuessingApp::close_app] )]
    window: nwg::Window,

    #[nwg_control(text: "50", size: (480, 25), position: (10, 10))]
    guess_edit: nwg::TextInput,

    #[nwg_control(text: "Guess", size: (480, 60), position: (10, 40))]
    #[nwg_events( OnButtonClick: [GuessingApp::guess] )]
    guess_button: nwg::Button,

    #[nwg_control(text: "", size: (10, 10), position: (10, 50))]
    #[nwg_events( OnButtonClick: [GuessingApp::guess] )]
    hidde_button: nwg::Button,

}


// turn my string to string slice so that it can be set to the text
fn to_str(value: &str) -> &str {
    value
}



impl GuessingApp {

    fn close_app(&self){
        nwg::stop_thread_dispatch();
        // stop thread on close cause hitting x wasnt stopping app without this dispatch 
    }
 
    fn guess(&self) {

        // had to set a blank text and then put a number inside of it and then use that number cause i wasnt able to mutate self.random number idk prob easier way to do this but this works.
        // this will be blank on first call 
        if self.hidde_button.text().is_empty() {
            // random number gets generated
            let random_number = rand::thread_rng().gen_range(1..=100);

            // turn it to string
            let my_string = random_number.to_string();

            // turn it to string slice
            let as_str = to_str(&my_string);

            // set that to hidebutton that was blank now will have the random number. 
            self.hidde_button.set_text(as_str);
        }


        // now that it isnt empty assign it to var and parse it to int 
        let same_guess = self.hidde_button.text().parse::<i32>().unwrap();
        
    

        // the users guess is parsed into int and matched to make sure its a number else get error message
        let user_guess = self.guess_edit.text().parse::<i32>();

        match user_guess {
            Ok(number) => {

                if number < same_guess {
                    nwg::simple_message("Try again!", &format!("to low!"));
                } else if number > same_guess {
                    nwg::simple_message("Try again!", &format!("to high!"));
        
                } else {
                    nwg::simple_message("You won!!!", &format!("Congrats!!"));
                    nwg::stop_thread_dispatch();
                }


            }
            Err(_err) => {
                nwg::simple_message("Not a number", &format!("Numbers only!"));
            }
        }


      

    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = GuessingApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}