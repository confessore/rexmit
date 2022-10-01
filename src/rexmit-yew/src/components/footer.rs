use yew::{Component, Html, html, Context, Properties};

pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="sm:w-[32rem] md:flex bg-slate-200 rounded-xl m-1 p-8 md:p-0 dark:bg-slate-800 bg-opacity-90">
                    <div class="pt-6 md:p-8 text-center md:text-left space-y-4">
                        <div>
                            <p class="text-lg font-medium">
                                {"Community and Career Solutions is a nonprofit disability employment service 
                                empowering people with the knowledge, connections and confidence required to 
                                embark on a new sustainable career"}
                            </p>
                        </div>
                        <div class="font-medium">
                            <div class="text-emerald-500 dark:text-emerald-400">
                                {"We are partnered with The Florida Division of Vocational Rehabilitation, 
                                the Agency for Persons with Disabilities, and the American Dream Employment Network"}
                            </div>
                            <div class="text-slate-700 dark:text-slate-500">
                                {"Our company is structured as an all inclusive integrated service offering adult employment 
                                services, youth programs, employability trainings, paid work experience, and a variety of 
                                assessments. Our services enhance a person’s ability to successfully achieve independence by 
                                establishing a lifelong career"}
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
