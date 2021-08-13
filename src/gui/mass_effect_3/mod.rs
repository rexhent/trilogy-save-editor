mod general;
mod plot;
mod plot_variable;
mod raw_plot;

pub use self::{general::*, plot::*, plot_variable::*, raw_plot::*};

use yew::prelude::*;

use super::RcUi;
use crate::{gui::raw_ui::RawUi, save_data::mass_effect_3::plot::PlotTable};

impl RawUi for RcUi<PlotTable> {
    fn view(&self, _: &str) -> yew::Html {
        html! { "// TODO: Link to Raw Plot" }
    }
}