use std::rc::Rc;

use anyhow::Error;
use indexmap::IndexMap;
use yew::{prelude::*, utils::NeqAssign};
use yew_agent::{Bridge, Bridged};

use crate::database_service::{Database, DatabaseService, Request, Response, Type};
use crate::gui::{
    components::{Tab, TabBar},
    mass_effect_1::Me1Plot,
    mass_effect_2::Me2Plot,
    mass_effect_3::PlotVariable,
    shared::{IntPlotType, PlotCategory},
    RcUi, Theme,
};
use crate::save_data::{
    mass_effect_3::plot_db::Me3PlotDb,
    shared::plot::{BitVec, PlotCategory as PlotCategoryDb},
};

pub enum Msg {
    PlotDb(Rc<Me3PlotDb>),
    Error(Error),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub booleans: RcUi<BitVec>,
    pub integers: IntPlotType,
    pub variables: RcUi<IndexMap<String, RcUi<i32>>>,
    pub onerror: Callback<Error>,
}

pub struct Me3Plot {
    props: Props,
    link: ComponentLink<Self>,
    _database_service: Box<dyn Bridge<DatabaseService>>,
    plot_db: Option<Rc<Me3PlotDb>>,
}

impl Component for Me3Plot {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut database_service =
            DatabaseService::bridge(link.callback(|response| match response {
                Response::Database(Database::Me3Plot(db)) => Msg::PlotDb(db),
                Response::Error(err) => Msg::Error(err),
                _ => unreachable!(),
            }));

        database_service.send(Request::Database(Type::Me3Plot));

        Me3Plot { props, link, _database_service: database_service, plot_db: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PlotDb(db) => {
                self.plot_db = Some(db);
                true
            }
            Msg::Error(err) => {
                self.props.onerror.emit(err);
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        if let Some(ref plot_db) = self.plot_db {
            let (booleans, integers, variables) =
                (&self.props.booleans, &self.props.integers, &self.props.variables);
            let Me3PlotDb {
                general,
                crew,
                romance,
                missions,
                citadel_dlc,
                normandy,
                appearances,
                weapons_powers,
                intel,
            } = plot_db.as_ref();

            let view_categories = |categories: &IndexMap<String, PlotCategoryDb>| {
                categories
                    .iter()
                    .map(|(title, category)| {
                        html! {
                            <PlotCategory
                                title={title.to_owned()}
                                booleans={RcUi::clone(booleans)}
                                integers={IntPlotType::clone(integers)}
                                category={category.clone()}
                            />
                        }
                    })
                    .collect::<Vec<_>>()
            };

            let categories = [
                ("Crew", crew),
                ("Romance", romance),
                ("Missions", missions),
                ("Normandy", normandy),
                ("Citadel DLC", citadel_dlc),
                ("Appearances", appearances),
            ];

            let categories = categories.iter().map(|(tab, categories)| {
                // Workaround for unused_braces warning
                #[allow(unused_braces)]
                (html_nested! {
                    <Tab title={tab.to_owned()}>
                        <div class="flex-auto flex flex-col gap-1">
                            { for view_categories(categories) }
                        </div>
                    </Tab>
                })
            });

            let weapons_powers = weapons_powers.iter().map(|(title, variable)| {
                html! {
                    <PlotVariable
                        title={title.to_owned()}
                        booleans={RcUi::clone(booleans)}
                        variables={RcUi::clone(variables)}
                        plot_variable={variable.clone()}
                    />
                }
            });

            html! {
                <TabBar>
                    <Tab title="General">
                        <PlotCategory
                            booleans={RcUi::clone(booleans)}
                            integers={IntPlotType::clone(integers)}
                            category={general.clone()}
                        />
                    </Tab>
                    { for categories }
                    <Tab title="Weapons / Powers">
                        <div class="flex-auto flex flex-col gap-1">
                            { for weapons_powers }
                        </div>
                    </Tab>
                    <Tab title="Intel">
                        <PlotCategory
                            booleans={RcUi::clone(booleans)}
                            integers={IntPlotType::clone(integers)}
                            category={intel.clone()}
                        />
                    </Tab>
                    <Tab title="Mass Effect 2" theme={Theme::MassEffect2}>
                        <Me2Plot
                            booleans={RcUi::clone(booleans)}
                            integers={IntPlotType::clone(integers)}
                            onerror={self.link.callback(Msg::Error)}
                        />
                    </Tab>
                    <Tab title="Mass Effect 1" theme={Theme::MassEffect1}>
                        <Me1Plot
                            me3_imported_me1={true}
                            booleans={RcUi::clone(booleans)}
                            integers={IntPlotType::clone(integers)}
                            onerror={self.link.callback(Msg::Error)}
                        />
                    </Tab>
                </TabBar>
            }
        } else {
            html! {
                <>
                    { "Loading database..." }
                    <hr class="border-t border-default-border" />
                </>
            }
        }
    }
}