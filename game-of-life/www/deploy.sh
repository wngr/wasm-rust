#!/bin/bash
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm

rm -rf ./dist
npm i
npm run build
rsync -avzP ./dist/* pallas:/mnt/user/web/wngr.de/

