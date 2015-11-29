#![allow(dead_code)]

use fccore::config::Config;
use fccore::configled::ConfigLed;
use fccore::configbutton::ConfigButton;
use simplelog::Log;

use time;

const TAG : &'static str = "core";
const LOG_DIR : &'static str = "./logs/";

pub struct Core {

    /**
     * Is the core alive
     */
    pub alive : bool,

    pub launched: bool,
    time_to_launch: Option<time::Tm>,
    
    /**
     * Base ARM requirement, safety switch must be switched to on
     */
    armed_switch : bool,
    
    /**
     * Second ARM requirement, a external request must arm the FC
     */
    armed_command : bool,
    
    /**
     * The armed status LED
     */
    armed_status_led : ConfigLed,
  
    /**
     * ARM safety switch on the device, if set to off position the FC will disable
     */
    armed_safety_switch : ConfigButton,
  
    /**
     * configuration for the core
     */
    config : Config,
  
    /**
     * Core log, stores log messages and timestamps
     */
    log : Log
}

impl Core {

    pub fn new(config_file : &str) -> Core {
        let config = Config::load(config_file);
        let mut core = Core {
            armed_switch: false,
            armed_command: false,
            alive: true,
            launched: false,
            time_to_launch: None,
            armed_status_led: ConfigLed::new(&config.armed_led),
            armed_safety_switch: ConfigButton::new(&config.arm_switch),
            log: Log::new(&format!("{}log{}", LOG_DIR, time::now().to_timespec().sec), config.log_config.log_limit),
            config: config
        };
        core.armed_changed();
        core
    }

    fn armed_changed(&mut self) {
        self.log.add(TAG, "armed_changed triggered");
        self.armed_status_led.set(self.armed());

        if !self.armed() {
            self.end_countdown();
        }
    }

    pub fn is_counting_down(&self) -> bool { self.time_to_launch.is_some() }
  
    /**
     * true if the device is fully armed
     */
    pub fn armed(&self) -> bool { self.armed_switch && self.armed_command }
  
    /**
     * true if an external arm is set
     */
    pub fn armed_cmd(&self) -> bool { self.armed_command }
    
    /**
     * true if the physical safety arm switch is armed
     */
    pub fn armed_switch(&self) -> bool { self.armed_switch }
  
    /**
     * Set the command ARM state
     * If the physical ARM button is off this will do nothing
     */
    pub fn set_armed_command(&mut self, state : bool) {

        if self.armed_switch {
            self.log_mut().add(TAG, &format!("ARM command request to set to {} handled at core", state));
            self.armed_command = state;
        } else {
            self.log_mut().add(TAG, "ARM command request ignored as armed_switch is disabled");
        }

        self.armed_changed();
    }

    pub fn countdown_time(&self) -> i64 {
        match self.time_to_launch {
            Some(time) => (time - time::now()).num_seconds(),
            None => 999999
        }
    }

    pub fn update(&mut self) {
    
        //Read from the physical safety
        let safety_state = self.armed_safety_switch.read_state();

        if safety_state && !self.armed_switch {
            self.log_mut().add(TAG, "physical safety switched to armed");
            self.armed_switch = true;
        } else if !safety_state && self.armed_switch {
            self.log_mut().add(TAG, "physical safety switched to disarmed");
            self.armed_switch = false;
        }

        //The ARM from command state is reset to false if the safety is off
        if !self.armed_switch && self.armed_command {
            self.log_mut().add(TAG, "set core armed_command to false as switch is false");
            self.armed_command = false;
            self.armed_changed();
        }

        if self.is_counting_down() && self.countdown_time() < 0 && !self.launched {
            self.launch();
            self.launched = true;
        }
    }

    fn launch(&mut self) {
        self.log_mut().add(TAG, "I would launch here");
    }

    pub fn begin_countdown(&mut self) {
        if !self.armed() {
            self.log_mut().add(TAG, "Cannot start countdown. Not ARMED");
        } else {

            if self.is_counting_down() {
                self.log_mut().add(TAG, "Resetting countdown");
            } else {
                self.log_mut().add(TAG, "Starting countdown");
            }

            self.launched = false;
            self.time_to_launch = Some(time::now() + time::Duration::seconds(self.config.countdown_time));
        }
    }

    pub fn end_countdown(&mut self) {
        if self.is_counting_down() {
            self.log_mut().add(TAG, "Manual countdown end");
            self.time_to_launch = None;
        }
    }

    /**
     * Get the core config struct
     */
    pub fn config(&self) -> &Config { &self.config }
    
    /**
     * Return the core log
     */
    pub fn log(&self) -> &Log { &self.log }
    
    /**
     * Return the core log as mutable
     */
     pub fn log_mut(&mut self) -> &mut Log { &mut self.log }
}
