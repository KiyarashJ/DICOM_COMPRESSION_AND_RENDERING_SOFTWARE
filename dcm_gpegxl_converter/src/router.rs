use dioxus::prelude::*;
use crate::pages::{Home::Home, Render::Render, RequestUser::Requestuser, Conversion::Conversion, RenderUserFile::Renderuserfile, HomeVsAnimation::HomeVsAnimation};
use dioxus_motion::prelude::*;


#[derive(Routable, Clone, Debug, PartialEq, MotionTransitions)]
pub enum Router {
    #[route("/")]
    #[transition(SlideDown)]
    HomeVsAnimation{},
    
    #[route("/home")]
    #[transition(SlideDown)]
    Home{},

    #[route("/user_req")]
    #[transition(SlideUp)]
    Requestuser{},

    //route needed to be assigned,

    #[route("/render")]
    #[transition(SlideRight)]
    Render{},
    // here you're gonna say that user enters its files or folder path,
    // route needed to be assigned,

    #[route("/conversion")]
    #[transition(SlideLeft)]
    Conversion{},
    // here you're gonna say that user enters its desired conversion format and thereafter its files,
    // route needed to be assigned,

    #[route("/render/userfile")]
    #[transition(Fade)]
    Renderuserfile{},
    // here you're gonna render the user files,
    // route needed to be assigned,
}