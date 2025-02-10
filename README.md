# BACKER

## A Rust command line tool for backing up a specific file b4 u wreck it

We already know what you did.  
We know why you searched for backup tools.  
That's why I made the tool.  I wrecked one-too-many important source files.  
This tool was built for Linux, but also works on Windoze.  

### Installation

If using the github tool:  `gh repo clone rlovell3/backer`  
If using git:  `git clone https://github.com/rlovell3/backer.git`  

Or be creative and create a new RUST project:  
`cargo new backer`  
`cd backer`  
Add chrono to your Cargo.toml:  
`cargo add chrono`  
Replace hello world source code with code from main.rs in this repository.  

Compile your project:  
`cargo build --release`  
The __backer__ executable will be in target/release directory.  

### Deploy backer into your path  

Copy the executable into a place on your path.  
I created a directory in my home directory for storing links to tools like this.  
`mkdir ~/bin`  

`cp target/release/backer ~/bin/`  

or create a symlink there so you can map to your stash of repositories:  
`ln -s path/to/repository/backer/target/release/backer ~/bin/backer`  

After you relaunch your shell, you should be set.

### Use

`backer filename`  
or  
`backer /path/to/filename`  

Example:  
`backer path/to/main.rs`  

Backer will provide confirmation of saving your @ss:  
`Backup path: .path/to/main.20230915:1517-34.backup.rs`  

Do that a few times, pausing a second or two between commands.  
Go look at the src dir after backing up your precious file a few times:  
`ls -al src/`  
`-rw-rw-r-- 1 rl rl 45 Sep 15 15:17 main.20230915_1517-22.backup.rs`  
`-rw-rw-r-- 1 rl rl 45 Sep 15 15:19 main.20230915_1519-12.backup.rs`  
`-rw-rw-r-- 1 rl rl 45 Sep 15 15:21 main.20230915_1521-17.backup.rs`  
`-rw-rw-r-- 1 rl rl   45 Sep 15 15:03 main.rs`  

Your backups are naturally sorted thanks to the ISO format of the timestamp, making your life easy.  
See how happy your are now that life just got easy?  

backup filenames are in this format: `path/filename.YYYYMMDD_HHMM-s.backup.file_extension`  

### Edit your .gitignore to exclude adding backups to your repository  

`vim .gitignore`  
Add the following line:  
`*.backup.*`  

You're welcome.  We've all done it.  
Just get into the habit of using backer before unleashing your inner wrecking ball.  

#### Build Notes  

2025-02-10  Updated .gitignore entry and a README typo.  
2024-10-26  Added seconds to timestamp and file extension to the backup filename.  
2023-12-09  Use `std::io::copy` instead of `fs::copy`.
