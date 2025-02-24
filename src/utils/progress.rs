use indicatif::{ProgressBar, ProgressStyle};

/// Wrapper for the ProgressBar struct
pub struct ProgressBarWrapper {
    progress_bar: ProgressBar,
    progress_style: ProgressStyle,
}

impl ProgressBarWrapper {
    ///
    ///
    /// # Arguments
    ///
    /// * `len`: u64 - The length of the progress bar.
    ///
    /// returns: ProgressBarWrapper
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::progress::ProgressBarWrapper;
    ///
    /// let progress_bar = ProgressBarWrapper::new(100);
    /// for _ in 0..100 {
    ///    progress_bar.inc(1);
    /// }
    /// progress_bar.finish();
    /// ```
    pub(crate) fn new(len: u64) -> Self {
        let progress_bar = ProgressBar::new(len);
        let progress_style = ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({msg})")
            .expect("Failed to create progress style")
            .progress_chars("#>-");

        progress_bar.set_style(progress_style.clone());

        Self {
            progress_bar,
            progress_style,
        }
    }

    /// increments the progress bar by `n`
    pub(crate) fn inc(&self, n: u64) {
        self.progress_bar.inc(n);
    }

    /// finishes the progress bar
    fn finish(&self) {
        self.progress_bar.finish();
    }

    /// finishes the progress bar with a message
    pub(crate) fn finish_with_message<T: ToString>(&self, message: T) {
        self.progress_bar.finish_with_message(message.to_string());
    }

    /// prints a message to the progress bar
    pub(crate) fn println<T: ToString>(&self, message: T) {
        self.progress_bar.println(message.to_string());
    }

    /// sets the message of the progress bar
    pub(crate) fn set_message<T: ToString>(&self, message: T) {
        self.progress_bar.set_message(message.to_string());
    }

    /// sets the position of the progress bar
    fn set_position(&self, pos: u64) {
        self.progress_bar.set_position(pos);
    }

    /// sets the length of the progress bar
    fn set_length(&self, len: u64) {
        self.progress_bar.set_length(len);
    }

    /// sets the style of the progress bar
    fn set_style(&self, style: ProgressStyle) {
        self.progress_bar.set_style(style);
    }

    /// sets the prefix of the progress bar
    fn set_prefix<T: ToString>(&self, prefix: T) {
        self.progress_bar.set_prefix(prefix.to_string());
    }
}
