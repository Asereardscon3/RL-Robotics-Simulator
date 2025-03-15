
use std::collections::HashMap;

/// Represents a state in the robotics environment.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    pub x: i32,
    pub y: i32,
}

/// Represents an action the robot can take.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
}

/// Q-Learning implementation for a robotic agent.
pub struct QLearningAgent {
    pub q_table: HashMap<(State, Action), f64>,
    pub learning_rate: f64,
    pub discount_factor: f64,
    pub epsilon: f64,
}

impl QLearningAgent {
    pub fn new(lr: f64, df: f64, eps: f64) -> Self {
        QLearningAgent {
            q_table: HashMap::new(),
            learning_rate: lr,
            discount_factor: df,
            epsilon: eps,
        }
    }

    pub fn get_q_value(&self, state: &State, action: &Action) -> f64 {
        *self.q_table.get(&(state.clone(), *action)).unwrap_or(&0.0)
    }

    pub fn choose_action(&self, state: &State) -> Action {
        // Epsilon-greedy strategy
        let actions = [Action::Up, Action::Down, Action::Left, Action::Right];
        // In a real app, we would use a random number generator here
        actions[0] // Simple placeholder for demonstration
    }

    pub fn update_q_value(&mut self, state: &State, action: Action, reward: f64, next_state: &State) {
        let current_q = self.get_q_value(state, &action);
        
        let actions = [Action::Up, Action::Down, Action::Left, Action::Right];
        let max_next_q = actions.iter()
            .map(|a| self.get_q_value(next_state, a))
            .fold(f64::NEG_INFINITY, f64::max);

        let new_q = current_q + self.learning_rate * (reward + self.discount_factor * max_next_q - current_q);
        self.q_table.insert((state.clone(), action), new_q);
    }
}

pub struct RoboticsEnvironment {
    pub goal: State,
}

impl RoboticsEnvironment {
    pub fn get_reward(&self, state: &State) -> f64 {
        if state == &self.goal {
            100.0
        } else {
            -1.0
        }
    }
}

fn main() {
    println!("--- RL Robotics Simulator Initializing ---");
    let mut agent = QLearningAgent::new(0.1, 0.9, 0.1);
    let env = RoboticsEnvironment { goal: State { x: 5, y: 5 } };

    let mut current_state = State { x: 0, y: 0 };
    
    for episode in 1..=100 {
        let action = agent.choose_action(&current_state);
        // Simulate movement
        let next_state = State { x: current_state.x + 1, y: current_state.y }; 
        let reward = env.get_reward(&next_state);
        
        agent.update_q_value(&current_state, action, reward, &next_state);
        current_state = next_state;
        
        if episode % 20 == 0 {
            println!("Episode {}: Agent at ({}, {})", episode, current_state.x, current_state.y);
        }
    }
    
    println!("Training completed.");
}
