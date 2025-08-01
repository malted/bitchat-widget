I wanted to know how many bitchat users are around me, all the time.

When I see someone online, I want to, you know, say hi. Or something.


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
