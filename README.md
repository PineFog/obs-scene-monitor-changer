# OBS-Scene-Monitor-Changer
## Purpose 
Change OBS scene based on which monitor your mouse cursor resides

## But why though?
If you wanna stream really boring content that lives across multiple monitors without the hassle of manually invoking scene changes, then maybe this is gluten-free.

### Usage
Runs as a command line app on the desktop running OBS. Modify `res/config.json` to add mappings between your OBS scenes and monitor names. You probably don't know your monitor names off hand _(I didn't)_, so app logs all monitors on startup and whenever your cursor crosses a monitor boundary. Automatically picks up config changes without relaunching app.

Requires OBS version >= 28. Make sure to enable OBS web sockets - `Tools -> obs-websocket Settings`. Be sure to set a password and copy pasta into `res/config.json`.

Selfishly written in a couple hours without any compassion for linux/mac users, proper testing, convention, error handling or scaled monitor resolutions.