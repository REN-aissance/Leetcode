#[derive(Default)]
struct RecentCounter {
    requests: Vec<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        Self {
            requests: Vec::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.requests.push(t);
        let start_time = t - 3000;
        let mut count = 0;
        for &request_time in self.requests.iter().rev() {
            if request_time < start_time {
                break;
            }
            count += 1;
        }
        count
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
