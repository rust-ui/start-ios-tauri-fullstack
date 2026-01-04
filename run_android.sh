#!/bin/bash

# Auto-detect Mac's IP and update tauri.android.conf.json (localhost doesn't work on mobile)
# - SERVER_IP: tells Android app where to connect
# - LEPTOS_SITE_ADDR=0.0.0.0: allows connections from Android emulator/device
SERVER_IP=$(ipconfig getifaddr en0)
sed -i '' "s|http://[0-9.]*:3000|http://${SERVER_IP}:3000|g" src-tauri/tauri.android.conf.json
echo "Updated SERVER_URL to http://${SERVER_IP}:3000"

LEPTOS_SITE_ADDR="0.0.0.0:3000" cargo tauri android dev
