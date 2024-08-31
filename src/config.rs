#[derive(serde::Deserialize)]
/// Configuration for the simulation
pub struct Config {
    /// Simulation configuration
    pub simulation: SimulationConfig,
    /// Quadrotor configuration
    pub quadrotor: QuadrotorConfig,
    /// Controller configuration
    pub controller: ControllerConfig,
    /// IMU configuration
    pub imu: ImuConfig,
    /// Maze configuration
    pub maze: MazeConfig,
    /// Camera configuration
    pub camera: CameraConfig,
    /// Mesh configuration
    pub mesh: MeshConfig,
    /// Planner schedule configuration
    pub planner_schedule: Vec<PlannerStep>,
    /// Use rerun.io for recording
    pub use_rerun: bool,
}

#[derive(serde::Deserialize)]
/// Configuration for a planner step
pub struct PlannerStep {
    /// Step number that the planner should be executed
    pub step: usize,
    /// Type of planner to use
    pub planner_type: String,
    /// Parameters for the planner
    pub params: serde_yaml::Value,
}

#[derive(serde::Deserialize)]
/// Configuration for the simulation
pub struct SimulationConfig {
    /// Control frequency in Hz
    pub control_frequency: f32,
    /// Simulation frequency in Hz
    pub simulation_frequency: f32,
    /// Log frequency in Hz
    pub log_frequency: f32,
    /// Duration of the simulation in seconds
    pub duration: f32,
}

#[derive(serde::Deserialize)]
/// Configuration for the quadrotor
pub struct QuadrotorConfig {
    /// Mass of the quadrotor in kg
    pub mass: f32,
    /// Gravity in m/s^2
    pub gravity: f32,
    /// Drag coefficient in Ns^2/m^2
    pub drag_coefficient: f32,
    /// Inertia matrix in kg*m^2
    pub inertia_matrix: [f32; 9],
}

#[derive(serde::Deserialize)]
/// Configuration for the controller
pub struct ControllerConfig {
    /// Position gains
    pub pos_gains: PIDGains,
    /// Attitude gains
    pub att_gains: PIDGains,
    /// Maximum integral error for position control
    pub pos_max_int: [f32; 3],
    /// Maximum integral error for attitude control
    pub att_max_int: [f32; 3],
}

#[derive(serde::Deserialize)]
/// Configuration for PID gains
pub struct PIDGains {
    /// Proportional gains
    pub kp: [f32; 3],
    /// Integral gains
    pub ki: [f32; 3],
    /// Derivative gains
    pub kd: [f32; 3],
}

#[derive(serde::Deserialize, Default)]
/// Configuration for the IMU
pub struct ImuConfig {
    /// Accelerometer noise standard deviation
    pub accel_noise_std: f32,
    /// Gyroscope noise standard deviation
    pub gyro_noise_std: f32,
    /// Accelerometer and gyroscope bias instability
    pub bias_instability: f32,
}

#[derive(serde::Deserialize)]
/// Configuration for the maze
pub struct MazeConfig {
    /// Upper bounds of the maze in meters (x, y, z)
    pub upper_bounds: [f32; 3],
    /// Lower bounds of the maze in meters (x, y, z)
    pub lower_bounds: [f32; 3],
    /// Number of obstacles in the maze
    pub num_obstacles: usize,
}

#[derive(serde::Deserialize)]
/// Configuration for the camera
pub struct CameraConfig {
    /// Camera resolution in pixels (width, height)
    pub resolution: (usize, usize),
    /// Camera field of view in degrees
    pub fov: f32,
    /// Camera near clipping plane in meters
    pub near: f32,
    /// Camera far clipping plane in meters
    pub far: f32,
}

#[derive(serde::Deserialize)]
/// Configuration for the mesh
pub struct MeshConfig {
    /// Division of the 2D mesh, the mesh will be division x division squares
    pub division: usize,
    /// Spacing between the squares in meters
    pub spacing: f32,
}

impl Config {
    /// Load configuration from a YAML file.
    /// # Arguments
    /// * `filename` - The name of the file to load.
    /// # Returns
    /// * The configuration object.
    /// # Errors
    /// * If the file cannot be read or the YAML cannot be parsed.
    pub fn from_yaml(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(filename)?;
        Ok(serde_yaml::from_str(&contents)?)
    }
}
