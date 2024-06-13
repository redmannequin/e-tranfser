use actix_web::{HttpRequest, HttpResponse};
use leptos::{component, view, IntoView};
use validation::{VLIDATE_AMOUNT, VLIDATE_PAYEE_EMAIL, VLIDATE_PAYER_EMAIL};

use crate::app::{
    component::{MyHtml, MyInput},
    payment_flow::PAYMENT_CREATE_PAGE,
};

pub async fn payment_form(_req: HttpRequest) -> HttpResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container-sm form-signin w-100 m-auto text-center" >
                    <form action={PAYMENT_CREATE_PAGE} method="post" >
                        <h1 class="text-light mb-3 fw-normal">Create Payment</h1>
                        <MyInput input_type="text" name="payer_full_name" label="Benefactor Name" required=true/>
                        <EmailInput name="payer_email" label="Benefactor Email" email={None} check=false endpoint={VLIDATE_PAYER_EMAIL}/>
                        <MyInput input_type="text" name="payee_full_name" label="Recipiant Name" required=true/>
                        <EmailInput name="payee_email" label="Recipiant Email" email={None} check=false endpoint={VLIDATE_PAYEE_EMAIL}/>
                        <AmountInput name="amount" label="Amount" amount={None} check=false endpoint={VLIDATE_AMOUNT}/>
                        <MyInput input_type="text" name="security_question" label="Security Question" required=true/>
                        <MyInput input_type="text" name="security_answer" label="Security Answer" required=true/>
                        <div class="input-group mb-3" >
                            <input
                                type="submit"
                                class="form-control btn btn-success"
                                value="CONTINUE"
                            />
                        </div>
                    </form>
                </div>
            </MyHtml>
        }
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.to_string())
}

pub mod validation {
    use actix_web::{web, HttpResponse};
    use concat_const::concat;
    use leptos::view;
    use serde::Deserialize;

    use crate::app::payment_flow::{
        z01_payment_form::{AmountInput, EmailInput},
        PAYMENT_FORM_PAGE,
    };

    pub const VLIDATE_PAYER_EMAIL: &str = concat!(PAYMENT_FORM_PAGE, "/validate/payer_email");
    pub const VLIDATE_PAYEE_EMAIL: &str = concat!(PAYMENT_FORM_PAGE, "/validate/payee_email");
    pub const VLIDATE_AMOUNT: &str = concat!(PAYMENT_FORM_PAGE, "/validate/amount");

    pub fn validation_scope() -> actix_web::Scope {
        web::scope("validate")
            .service(web::resource("payer_email").post(payer_email))
            .service(web::resource("payee_email").post(payee_email))
            .service(web::resource("amount").post(amount))
    }

    #[derive(Debug, Deserialize)]
    struct PayerEmail {
        payer_email: String,
    }

    async fn payer_email(form: web::Form<PayerEmail>) -> HttpResponse {
        let email = form.0.payer_email;
        let html = leptos::ssr::render_to_string(move || {
            view! {
                <EmailInput name="payer_email" label="Benefactor Email" email={Some(email)} check=true endpoint={VLIDATE_PAYER_EMAIL}/>
            }
        });

        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html.to_string())
    }

    #[derive(Debug, Deserialize)]
    struct PayeeEmail {
        payee_email: String,
    }

    async fn payee_email(form: web::Form<PayeeEmail>) -> HttpResponse {
        let email = form.0.payee_email;
        let html = leptos::ssr::render_to_string(move || {
            view! {
                <EmailInput name="payee_email" label="Recipiant Email" email={Some(email)} check=true endpoint={VLIDATE_PAYEE_EMAIL}/>
            }
        });

        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html.to_string())
    }

    #[derive(Debug, Deserialize)]
    struct AmountFormData {
        amount: u32,
    }

    async fn amount(form: web::Form<AmountFormData>) -> HttpResponse {
        let amount = form.0.amount;
        let html = leptos::ssr::render_to_string(move || {
            view! {
                <AmountInput name="amount" label="Amount" amount={Some(amount)} check=true endpoint={VLIDATE_AMOUNT}/>
            }
        });

        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html.to_string())
    }
}

#[component]
fn amount_input(
    name: &'static str,
    label: &'static str,
    amount: Option<u32>,
    check: bool,
    endpoint: &'static str,
) -> impl IntoView {
    let (class, err_msg) = match (check, amount.unwrap_or_default() >= 100) {
        (true, false) => ("form-control is-invalid", "A minimum of 100 is required..."),
        (false, false) => ("form-control", ""),
        (_, _) => ("form-control is-valid", ""),
    };

    view! {
        <div class="form-floating mb-3 has-validation" hx-target="this" hx-swap="outerHTML">
            <input
                class={class}
                type="text"
                inputmode="numeric"
                pattern="[0-9]*"
                id={name}
                name={name}
                hx-post={endpoint}
                value={amount}
                required
                data-1p-ignore
            />
            <label for="user_email">{label}</label>
            <div class="invalid-feedback">
                {err_msg}
            </div>
        </div>
    }
}

#[component]
fn email_input(
    name: &'static str,
    label: &'static str,
    email: Option<String>,
    check: bool,
    endpoint: &'static str,
) -> impl IntoView {
    let (class, err_msg) = match (
        check,
        email_address::EmailAddress::is_valid(email.as_deref().unwrap_or_default()),
    ) {
        (true, false) => (
            "form-control is-invalid",
            "Please enter a valid email address...",
        ),
        (false, false) => ("form-control", ""),
        (_, _) => ("form-control is-valid", ""),
    };

    view! {
        <div class="form-floating mb-3 has-validation" hx-target="this" hx-swap="outerHTML">
            <input
                class={class}
                type="email"
                id={name}
                name={name}
                data-1p-ignore
                hx-post={endpoint}
                value={email}
                required=true
            />
            <label for="user_email">{label}</label>
            <div class="invalid-feedback">
                {err_msg}
            </div>
        </div>
    }
}
