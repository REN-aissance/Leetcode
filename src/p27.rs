use std::collections::VecDeque;

#[derive(Default)]
struct RecentCounter {
    requests: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        Self {
            requests: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.requests.push_back(t);
        let start_time = t - 3000;
        while let Some(request_time) = self.requests.front() {
            if *request_time < start_time {
                self.requests.pop_front();
            } else {
                break;
            }
        }
        self.requests.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::RecentCounter;

    #[test]
    fn test_example_1() {
        let mut recent_counter = RecentCounter::new();
        assert_eq!(recent_counter.ping(1), 1);
        assert_eq!(recent_counter.ping(100), 2);
        assert_eq!(recent_counter.ping(3001), 3);
        assert_eq!(recent_counter.ping(3002), 3);
    }
}
