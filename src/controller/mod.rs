pub trait Controller {
    fn initialize();
    fn process_events();
    fn should_exit();
}
