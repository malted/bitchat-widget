# A little widget for [bitchat](https://github.com/permissionlesstech/bitchat)

I wanted to know how many bitchat users are around me, all the time...

<img width="826" height="55" alt="Screenshot 2025-07-31 at 11 50 01 PM" src="https://github.com/user-attachments/assets/6724a2dc-ba68-417d-bed7-4c4353e6ddd9" />

...so when I see someone online, I can, you know, say hi. Or something.

<img width="150" height="160" alt="image" src="https://github.com/user-attachments/assets/818aab4f-268e-4851-80a0-ddb30e16f328" />


 ## Install

 ```sh
 cargo install --git https://github.com/malted/bitchat-widget

 sudo tee /Library/LaunchDaemons/dev.malted.bitchat-widget.plist <<EOF
 <?xml version="1.0" encoding="UTF-8"?>
 <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
 <plist version="1.0">
 <dict>
     <key>Label</key>
     <string>dev.malted.bitchat-widget</string>

     <key>ProgramArguments</key>
     <array>
         <string>$HOME/.cargo/bin/bitchat-widget</string>
     </array>

     <key>RunAtLoad</key>
     <true/>

     <key>KeepAlive</key>
     <true/>
 </dict>
 </plist>
 EOF

 sudo chmod 644 /Library/LaunchDaemons/dev.malted.bitchat-widget.plist

 sudo launchctl load /Library/LaunchDaemons/dev.malted.bitchat-widget.plist
 ```

You'll need to grant `~/.cargo/bin/bitchat-widget` accessability permissions, because it accesses bitchat's UI element tree. Go to `System Settings > Privacy & Security > Accessability > +`, then `⌘⇧H` to go to `~`, then `⌘⇧.` to view dotfiles, then select `.cargo/bitchat-widget`.
