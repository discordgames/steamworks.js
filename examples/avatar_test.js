const fs = require('fs');
const path = require('path');
const steamworks = require('../index');
const sharp = require('sharp');

// Initialize the Steam API
// Replace 480 with your app ID
const client = steamworks.init(480);

// Get the current user's Steam ID
const mySteamId = client.localplayer.getSteamId();
console.log(`My Steam ID: ${mySteamId.steamId64}`);

// Get the current user's persona name
const myName = client.localplayer.getName();
console.log(`My name: ${myName}`);

// Get the current user's avatar
console.log('Fetching my avatar...');
const smallAvatar = client.friends.getSmallFriendAvatar(mySteamId.steamId64);
const mediumAvatar = client.friends.getMediumFriendAvatar(mySteamId.steamId64);
const largeAvatar = client.friends.getLargeFriendAvatar(mySteamId.steamId64);

// Create a directory for the avatars
const avatarDir = path.join(__dirname, 'avatars');
if (!fs.existsSync(avatarDir)) {
    fs.mkdirSync(avatarDir);
}

// Helper function to convert RGBA buffer to PNG
async function convertRGBAtoPNG(rgbaBuffer, width, height, outputPath) {
    try {
        await sharp(rgbaBuffer, {
            raw: {
                width: width,
                height: height,
                channels: 4
            }
        })
            .png()
            .toFile(outputPath);
        console.log(`Image saved to ${outputPath}`);
        return true;
    } catch (error) {
        console.error(`Error converting image: ${error.message}`);
        return false;
    }
}

// Save the avatars to disk
if (smallAvatar) {
    // Save the raw RGBA data for reference
    fs.writeFileSync(path.join(avatarDir, `${myName}_small.rgba`), smallAvatar);
    console.log(`Small avatar raw data saved to ${path.join(avatarDir, `${myName}_small.rgba`)}`);
    console.log(`Small avatar size: ${smallAvatar.length} bytes`);

    // Convert to PNG (32x32 pixels)
    convertRGBAtoPNG(
        smallAvatar,
        32, 32,
        path.join(avatarDir, `${myName}_small.png`)
    );
} else {
    console.log('Small avatar not available');
}

if (mediumAvatar) {
    // Save the raw RGBA data for reference
    fs.writeFileSync(path.join(avatarDir, `${myName}_medium.rgba`), mediumAvatar);
    console.log(`Medium avatar raw data saved to ${path.join(avatarDir, `${myName}_medium.rgba`)}`);
    console.log(`Medium avatar size: ${mediumAvatar.length} bytes`);

    // Convert to PNG (64x64 pixels)
    convertRGBAtoPNG(
        mediumAvatar,
        64, 64,
        path.join(avatarDir, `${myName}_medium.png`)
    );
} else {
    console.log('Medium avatar not available');
}

if (largeAvatar) {
    // Save the raw RGBA data for reference
    fs.writeFileSync(path.join(avatarDir, `${myName}_large.rgba`), largeAvatar);
    console.log(`Large avatar raw data saved to ${path.join(avatarDir, `${myName}_large.rgba`)}`);
    console.log(`Large avatar size: ${largeAvatar.length} bytes`);

    // Convert to PNG (184x184 pixels)
    convertRGBAtoPNG(
        largeAvatar,
        184, 184,
        path.join(avatarDir, `${myName}_large.png`)
    );
} else {
    console.log('Large avatar not available');
}

// Register a callback for avatar image loaded
const avatarCallback = client.callback.register(
    steamworks.SteamCallback.AvatarImageLoaded,
    (data) => {
        console.log('Avatar image loaded callback received:');
        console.log(`  Steam ID: ${data.steam_id}`);
        console.log(`  Image: ${data.image}`);
        console.log(`  Width: ${data.width}`);
        console.log(`  Height: ${data.height}`);
    }
);

// Callbacks are automatically run by the library
console.log('Waiting for avatar callbacks...');
setTimeout(() => {
    avatarCallback.disconnect();
    console.log('Done!');
}, 10000); 