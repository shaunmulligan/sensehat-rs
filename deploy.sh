cross build --example main --target=armv7-unknown-linux-gnueabihf 
git add .
git commit -m "release to fleet"
git push resin master