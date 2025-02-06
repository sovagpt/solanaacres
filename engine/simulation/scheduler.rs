use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

pub struct Scheduler {
    tasks: BinaryHeap<ScheduledTask>,
    running_tasks: HashMap<Uuid, TaskHandle>,
}

#[derive(Debug)]
pub struct ScheduledTask {
    id: Uuid,
    execution_time: f32,
    priority: i32,
    task: Box<dyn Task>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            tasks: BinaryHeap::new(),
            running_tasks: HashMap::new(),
        }
    }

    pub fn schedule_task(&mut self, task: impl Task + 'static, delay: f32, priority: i32) -> Uuid {
        let id = Uuid::new_v4();
        let scheduled_task = ScheduledTask {
            id,
            execution_time: delay,
            priority,
            task: Box::new(task),
        };
        self.tasks.push(scheduled_task);
        id
    }

    pub fn update(&mut self, current_time: f32) {
        while let Some(task) = self.tasks.peek() {
            if task.execution_time <= current_time {
                if let Some(task) = self.tasks.pop() {
                    task.task.execute();
                }
            } else {
                break;
            }
        }
    }
}