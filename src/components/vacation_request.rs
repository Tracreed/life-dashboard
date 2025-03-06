use leptos::*;
use leptos_router::*;
use chrono::NaiveDate;

#[component]
pub fn VacationRequestForm() -> impl IntoView {
    // State management using Leptos signals
    let (start_date, set_start_date) = create_signal(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
    let (end_date, set_end_date) = create_signal(NaiveDate::from_ymd_opt(2024, 1, 7).unwrap());
    let (error, set_error) = create_signal(String::new());
    let (success, set_success) = create_signal(false);

    // Action for form submission (Leptos innovation!)
    let submit_vacation_request = create_action(move |&()| async move {
        let request = crate::routes::VacationRequest {
            start_date: start_date.get(),
            end_date: end_date.get(),
        };

        match leptos_axum::post("/vacation-request", request).await {
            Ok(_) => {
                set_success.set(true);
                set_error.set(String::new());
            }
            Err(e) => {
                set_error.set(format!("Submission failed: {}", e));
                set_success.set(false);
            }
        }
    });

    view! {
        <div class="card w-96 bg-base-100 shadow-xl">
            <div class="card-body">
                <h2 class="card-title">Vacation Request</h2>
                <form on:submit=move |ev| {
                    ev.prevent_default();
                    submit_vacation_request.dispatch(());
                }>
                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">Start Date</span>
                        </label>
                        <input 
                            type="date" 
                            class="input input-bordered" 
                            prop:value=move || start_date.get().to_string()
                            on:change=move |ev| {
                                if let Ok(date) = NaiveDate::parse_from_str(&ev.target().value(), "%Y-%m-%d") {
                                    set_start_date.set(date);
                                }
                            }
                        />
                    </div>
                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">End Date</span>
                        </label>
                        <input 
                            type="date" 
                            class="input input-bordered" 
                            prop:value=move || end_date.get().to_string()
                            on:change=move |ev| {
                                if let Ok(date) = NaiveDate::parse_from_str(&ev.target().value(), "%Y-%m-%d") {
                                    set_end_date.set(date);
                                }
                            }
                        />
                    </div>
                    <div class="form-control mt-6">
                        <button 
                            type="submit" 
                            class="btn btn-primary"
                            disabled=move || submit_vacation_request.pending().get()
                        >
                            {move || if submit_vacation_request.pending().get() { "Submitting..." } else { "Submit Request" }}
                        </button>
                    </div>
                </form>

                {move || if !error.get().is_empty() {
                    view! { <div class="alert alert-error">{error.get()}</div> }
                } else if success.get() {
                    view! { <div class="alert alert-success">Vacation request submitted successfully!</div> }
                } else {
                    view! {}
                }}
            </div>
        </div>
    }
}