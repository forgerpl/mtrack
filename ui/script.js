const apiBaseUrl = '/api';

const playButton = document.getElementById('play-button');
const prevButton = document.getElementById('prev-button');
const nextButton = document.getElementById('next-button');
const stopButton = document.getElementById('stop-button');
const allButton = document.getElementById('all-button');
const playlistButton = document.getElementById('playlist-button');

const currentSongElement = document.getElementById('current-song');
const playlistNameElement = document.getElementById('playlist-name');
const playlistListElement = document.getElementById('playlist-list');

playButton.addEventListener('click', playTrack);
prevButton.addEventListener('click', prevTrack);
nextButton.addEventListener('click', nextTrack);
stopButton.addEventListener('click', stopTrack);
allButton.addEventListener('click', switchToAllSongsPlaylist);
playlistButton.addEventListener('click', switchToConfiguredPlaylist);

function playTrack() {
  fetch(`${apiBaseUrl}/play`, { method: 'POST' })
    .then(response => response.json())
    .then(data => console.log(`Playing track: ${data.track}`))
    .catch(error => console.error('Error playing track:', error));
}

function prevTrack() {
  fetch(`${apiBaseUrl}/prev`, { method: 'POST' })
    .then(response => response.json())
    .then(data => console.log(`Previous track: ${data.track}`))
    .catch(error => console.error('Error previous track:', error));
}

function nextTrack() {
  fetch(`${apiBaseUrl}/next`, { method: 'POST' })
    .then(response => response.json())
    .then(data => console.log(`Next track: ${data.track}`))
    .catch(error => console.error('Error next track:', error));
}

function stopTrack() {
  fetch(`${apiBaseUrl}/stop`, { method: 'POST' })
    .then(response => response.json())
    .then(data => console.log('Stopped playing'))
    .catch(error => console.error('Error stopping track:', error));
}

function switchToAllSongsPlaylist() {
  fetch(`${apiBaseUrl}/all`, { method: 'POST' })
    .then(response => response.json())
    .then(data => {
      playlistNameElement.textContent = 'All Songs';
      currentSongElement.textContent = data.track;
    })
    .catch(error => console.error('Error switching to all songs playlist:', error));
}

function switchToConfiguredPlaylist() {
  fetch(`${apiBaseUrl}/playlist`, { method: 'POST' })
    .then(response => response.json())
    .then(data => {
      playlistNameElement.textContent = 'Configured Playlist';
      currentSongElement.textContent = data.track;
    })
    .catch(error => console.error('Error switching to configured playlist:', error));
}

// Initialize the player state
fetch(`${apiBaseUrl}/state`, { method: 'GET' })
  .then(response => response.json())
  .then(updateState)
  .catch(error => console.error('Error initializing player state:', error));
  
  const socket = new WebSocket(((window.location.protocol === "https:") ? "wss://" : "ws://") + window.location.host + "/ws");

  socket.onmessage = (event) => {
    console.log(event.data);
    const data = JSON.parse(event.data);
    updateState(data);
  };

function updateState(data) {
  if (data.state === 'playing') {
    updatePlayingState(data.songname, data.pos, data.playlist);
  } else if (data.state === 'stopped') {
    updateStoppedState(data.songname, data.pos, data.playlist);
  }
}

  // Update playing state UI
function updatePlayingState(song, position, playlist) {
  document.getElementById('player-state').textContent = "playing";
  document.getElementById('current-song').textContent = song;
  document.getElementById('playlist-name').textContent = `Playlist ${position + 1}/${playlist.length}`;
  const playlistList = document.getElementById('playlist-list');
  playlistList.innerHTML = '';
  playlist.forEach((song, index) => {
    const li = document.createElement('li');
    li.textContent = song;
    if (index === position) {
      li.classList.add('active');
    }
    playlistList.appendChild(li);
  });
}

// Update stopped state UI
function updateStoppedState(song, position, playlist) {
  document.getElementById('player-state').textContent = "stopped";
  document.getElementById('current-song').textContent = song;
  document.getElementById('playlist-name').textContent = `Playlist ${position + 1}/${playlist.length}`;
  const playlistList = document.getElementById('playlist-list');
  playlistList.innerHTML = '';
  playlist.forEach((song, index) => {
    const li = document.createElement('li');
    li.textContent = song;
    if (index === position) {
      li.classList.add('active');
    }
    playlistList.appendChild(li);
  });
}