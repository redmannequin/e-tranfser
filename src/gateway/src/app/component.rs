use leptos::{component, view, Children, IntoView};

#[component]
pub fn my_input(
    input_type: &'static str,
    name: &'static str,
    label: &'static str,
    required: bool,
) -> impl IntoView {
    view! {
        <div class="form-floating mb-3" >
            <input
                type={input_type}
                id={name}
                name={name}
                class="form-control"
                data-1p-ignore
                required={required}
            />
            <label for={name}>{label}</label>
        </div>
    }
}

#[component]
pub fn navbar() -> impl IntoView {
    view! {
        <nav class="navbar bg-success" >
            <div class="container" >
                <h1 class="navbar-brand text-light" href="#" >e-transfer</h1>
            </div>
        </nav>
    }
}

#[component]
pub fn my_html(children: Children) -> impl IntoView {
    view! {
        <html lang="en" data-bs-theme="dark" >

        <head>
            <meta charset="utf-8"/>
            <meta name="viewport" content="width=device-width, initial-scale=1"/>
            <title>e-transfer</title>
            <link
                href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css"
                rel="stylesheet"
                integrity="sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN"
                crossorigin="anonymous"
            />
            <style>
                "
                .form-signin {
                    max-width: 450px;
                    padding: 1rem;
                }
                "
            </style>
        </head>

        <body>
            <Navbar />
            {children()}
            <script
                src="https://cdn.jsdelivr.net/npm/@popperjs/core@2.11.8/dist/umd/popper.min.js"
                integrity="sha384-I7E8VVD/ismYTF4hNIPjVp/Zjvgyol6VFvRkX/vR+Vc4jQkC+hVqc2pM8ODewa9r"
                crossorigin="anonymous"
            ></script>
            <script
                src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.min.js"
                integrity="sha384-BBtl+eGJRgqQAUMxJ7pMwbEyER4l1g+O15P+16Ep7Q9Q+zqX6gSbd85u4mG4QzX+"
                crossorigin="anonymous"
            ></script>
            <script
                src="https://unpkg.com/htmx.org@1.9.6"
                integrity="sha384-FhXw7b6AlE/jyjlZH5iHa/tTe9EpJ1Y55RjcgPbjeWMskSxZt1v9qkxLJWNJaGni"
                crossorigin="anonymous"
            ></script>
        </body>

        </html>
    }
}
