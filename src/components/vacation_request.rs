use leptos::*;
use leptos_router::*;
use crate::services::database::VacationRequestService;
use crate::services::email::EmailService;
use crate::utils::config::SmtpConfig;

#[component]
pub fn VacationRequestForm() -> impl IntoView {
    let (start_date, set_start_date) = create_signal(String::new());
    let (end_date, set_end_date) = create_signal(String::new());
    let (error, set_error) = create_signal(Option::<String>::None);

    let submit_vacation = create_action(move |_| async move {
        let start = start_date.get();
        let end = end_date.get();

        // Validate dates
        if start.is_empty() || end.is_empty() {
            set_error.set(Some("Please select both start and end dates".to_string()));
            return;
        }

        // In a real app, you'd inject these dependencies
        let db_service = VacationRequestService::new();
        let email_service = EmailService::new(SmtpConfig::load().unwrap());

        match db_service.create_request(&start, &end).await {
            Ok(_) => {
                // Send email (in real scenario, get boss name from config)
                if let Err(e) = email_service.send_vacation_request(
                    "John Doe", // Replace with actual boss name
                    "Employee Name", // Replace with actual employee name
                    &start, 
                    &end
                ) {
                    set_error.set(Some(format!("Email send failed: {}", e)));
                    return;
                }

                // Reset form or redirect
                set_start_date.set(String::new());
                set_end_date.set(String::new());
                set_error.set(None);
            },
            Err(e) => {
                set_error.set(Some(format!("Submission failed: {}", e)));
            }
        }
    });

    view! {
        <div class="card w-96 bg-base-100 shadow-xl">
            <div class="card-body">
                <h2 class="card-title">Vacation Request</h2>
                <form on:submit=move |ev| {
                    ev.prevent_default();
                    submit_vacation.dispatch(());
                }>
                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">Start Date</span>
                        </label>
                        <input 
                            type="date" 
                            prop:value=start_date 
                            on:input=move |ev| {
                                set_start_date.set(event_target_value(&ev));
                            }
                            class="input input-bordered"
                        />
                    </div>

                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">End Date</span>
                        </label>
                        <input 
                            type="date" 
                            prop:value=end_date 
                            on:input=move |ev| {
                                set_end_date.set(event_target_value(&ev));
                            }
                            class="input input-bordered"
                        />
                    </div>

                    {move || error.get().map(|e| view! {
                        <div class="alert alert-error">
                            {e}
                        </div>
                    })}

                    <div class="form-control mt-6">
                        <button 
                            type="submit" 
                            class="btn btn-primary"
                            disabled=move || submit_vacation.pending().get()
                        >
                            {move || if submit_vacation.pending().get() 
                                { "Submitting..." } 
                                else { "Submit Request" }
                            }
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}