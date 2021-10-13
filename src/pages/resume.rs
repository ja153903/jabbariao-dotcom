use yew::{classes, html, Component, ComponentLink, Html, ShouldRender};

pub struct Experience {
    pub company: String,
    pub job_title: String,
    pub city: String,
    pub state: String,
    pub start_date: String,
    pub end_date: String,
    pub description: String,
}

pub struct Education {
    pub school: String,
    pub degree: String,
    pub gpa: String,
    pub start_date: String,
    pub end_date: String,
    pub description: String,
}

pub struct Resume;

impl Component for Resume {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Resume {}
    }

    fn view(&self) -> Html {
        let experiences: Vec<Experience> = vec![
            Experience {
                company: String::from("BentoBox"),
                job_title: String::from("Software Engineer II"),
                city: String::from("New York"),
                state: String::from("NY"),
                start_date: String::from("Mar. 2021"),
                end_date: String::from("Present"),
                description: String::from(
                    "Building out tools in React and Python for restaurants to thrive online",
                ),
            },
            Experience {
                company: String::from("E*TRADE"),
                job_title: String::from("Software Engineer"),
                city: String::from("Jersey City"),
                state: String::from("NJ"),
                start_date: String::from("Jul. 2019"),
                end_date: String::from("Mar. 2021"),
                description: String::from("Built products for guided investing in React and Java"),
            },
        ];

        let education: Vec<Education> = vec![
            Education {
                school: String::from("New York University"),
                degree: String::from("Master of Science in Computer Science"),
                gpa: String::from("3.9"),
                start_date: String::from("Jan. 2018"),
                end_date: String::from("Jan. 2020"),
                description: String::from("Fiddled with machine learning and distributed systems"),
            },
            Education {
                school: String::from("Baruch College"),
                degree: String::from("Bachelor of Arts in Mathematics"),
                gpa: String::from("3.94"),
                start_date: String::from("Jan. 2014"),
                end_date: String::from("Jul. 2017"),
                description: String::from("Resident math and stats wizard"),
            },
        ];

        html!(
            <div class=classes!("container", "mx-auto", "text-left", "font-mono")>
                <div class=classes!("py-8")>
                    <h2 class=classes!("text-2xl")>{ "Experience" }</h2>
                    <ul>
                        { for experiences.iter().map(|e| self.view_experience_entry(e)) }
                    </ul>
                </div>
                <div class=classes!("py-4")>
                    <h2 class=classes!("text-2xl")>{ "Education" }</h2>
                    <ul>
                        { for education.iter().map(|e| self.view_education_entry(e)) }
                    </ul>
                </div>
            </div>
        )
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        unimplemented!()
    }
}

impl Resume {
    fn view_experience_entry(&self, entry: &Experience) -> Html {
        html!(
            <li>
                <div class=classes!("py-2")>
                    <h3 class=classes!("text-xl", "py-2")>{ &entry.company }</h3>
                    <p class=classes!("italic")>{ format!("{}, {}, {}, {} - {}", &entry.job_title, &entry.city, &entry.state, &entry.start_date, &entry.end_date) }</p>
                    <p class=classes!("py-2")>{ &entry.description }</p>
                </div>
            </li>
        )
    }

    fn view_education_entry(&self, entry: &Education) -> Html {
        html!(
            <li>
                <div class=classes!("py-2")>
                    <h3 class=classes!("text-xl", "py-2")>{ &entry.school }</h3>
                    <p class=classes!("italic")>{ format!("{}, GPA: {}, {} - {}", &entry.degree, &entry.gpa, &entry.start_date, &entry.end_date) }</p>
                    <p class=classes!("py-2")>{ &entry.description }</p>
                </div>
            </li>
        )
    }
}
