# BACKER
### A Rust command line tool for backing up a specific file b4 u wreck it.


We already know what you did.  
We know why you searched for backup tools.  
That's why I made the tool.  I wrecked one-too-many important source files.  


### Installation
If using the github tool:  `gh repo clone rlovell3/backer`  


Or be creative and create a new RUST project:  
`cargo new backer`  
`cd backer`  
Add chrono to your Cargo.toml:  
`cargo add chrono`  
Replace the hello world source code with the code from this repository.  
Have your favorite chatbot explain the source code if you don't fully understand it.  Never run code that you didn't write if you don't understand it.  

Compile your project:  
`cargo build --release`  
The executable will be in the target/release dir if you didn't already know that.  

## Deploy backer into your path  
Copy the executable into a place on your path.  
I created a dir in my home dir just for storing cool tools like this.  
`mkdir ~/bin`  
`cp target/release/backer ~/bin/`  
Be sure to add that path to your shell configuration.  Ask your fave AI chatbot if you don't know how.  
After you relaunch your shell, you should be set.

## Use:
My command prompt is in the root directory of my project.  
The file I want to quickly back up is src/main.rs.  
Execute backer with path/filename as the argument.  
`backer src/main.rs`   
Backer will provide confirmation of saving your @ss:  
`Backup path: .src/main.20230915:1517.backup`   

Do that a few times, pausing a minute or two between commands.  
Go look at the src dir after backing up your precious file a few times:  
`ls -al src/`   
`-rw-rw-r-- 1 rl rl 45 Sep 15 15:17 main.20230915:1517.backup`  
`-rw-rw-r-- 1 rl rl 45 Sep 15 15:19 main.20230915:1519.backup`  
`-rw-rw-r-- 1 rl rl 45 Sep 15 15:21 main.20230915:1521.backup`  
`-rw-rw-r-- 1 rl rl   45 Sep 15 15:03 main.rs`  
Your backups are naturally sorted, making your life easy. 
See how happy your are now that life just got easy?   

# Edit your .gitignore to exclude adding backups to your repository:  
`nano .gitignore`  
Add the following line:  
`*.backup`  

You're welcome.  We've all done it.  
Just get into the habit of using backer before unleashing your inner wrecking ball.  




