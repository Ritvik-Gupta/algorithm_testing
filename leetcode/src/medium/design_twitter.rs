use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

struct User {
    tweets: Vec<(usize, i32)>,
    followee_ids: HashSet<i32>,
}

const RECENT_TWEETS_SIZE: usize = 10;

impl User {
    fn new(user_id: i32) -> Self {
        let mut follower_ids = HashSet::new();
        follower_ids.insert(user_id);
        Self {
            tweets: Vec::new(),
            followee_ids: follower_ids,
        }
    }
}

struct Twitter {
    timestamp: usize,
    users: HashMap<i32, User>,
}

impl Twitter {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            timestamp: 0,
            users: HashMap::new(),
        }
    }

    fn get_user(&mut self, user_id: i32) -> &mut User {
        self.users
            .entry(user_id)
            .or_insert_with(|| User::new(user_id))
    }

    #[allow(dead_code)]
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let timestamp = self.timestamp;
        self.timestamp += 1;

        self.get_user(user_id).tweets.push((timestamp, tweet_id));
    }

    #[allow(dead_code)]
    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let followee_ids: *const _ = &self.get_user(user_id).followee_ids;

        let mut news_heap = BinaryHeap::with_capacity(RECENT_TWEETS_SIZE + 1);

        unsafe { &*followee_ids }
            .iter()
            .flat_map(|&followee_id| self.get_user(followee_id).tweets.clone())
            .for_each(|tweet| {
                news_heap.push(Reverse(tweet));
                if news_heap.len() > RECENT_TWEETS_SIZE {
                    news_heap.pop();
                }
            });

        let mut news_feed = Vec::with_capacity(news_heap.len());
        while let Some(Reverse((_, tweet))) = news_heap.pop() {
            news_feed.push(tweet);
        }
        news_feed.into_iter().rev().collect()
    }

    #[allow(dead_code)]
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.get_user(follower_id).followee_ids.insert(followee_id);
    }

    #[allow(dead_code)]
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.get_user(follower_id).followee_ids.remove(&followee_id);
    }
}
