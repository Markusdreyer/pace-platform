'use client';

import { useState } from 'react';
import GoogleMapReact from 'google-map-react';
import 'tailwindcss/tailwind.css';

const Marker = ({ text }: any) => <div className="p-1 bg-red-500 rounded-full">{text}</div>;

interface LocationUpdate {
    userId: string;
    timestamp: number;
    coordinates: {
        lat: number;
        long: number;
    };
}

interface Params {
  raceId: string;

}

const REMOTE_URL = 'wss://websockets.fly.dev/';
const LOCAL_URL  = 'ws://localhost:8080/';

export default function MapPage({params}: any) {
  const raceId = params.raceId
  const [locations, setLocations] = useState<LocationUpdate>(Object.create(null));
  const [center, setCenter] = useState({ lat: 59.95, lng: 30.33 });
  const ws = new WebSocket(LOCAL_URL + raceId);

  ws.onopen = () => {  
    console.log("websocket connection established");
  };

  ws.onmessage = (event: any) => {
    const json = JSON.parse(event.data);
    console.log(json);
    setLocations(prevLocations => ({ 
      ...prevLocations, 
      [json.userId]: { lat: json.coordinates.lat, lng: json.coordinates.long }
    }));

    // Set the map center to the latest location
    setCenter({ lat: json.coordinates.lat, lng: json.coordinates.long });
  };

  return (
    <div style={{ height: '100vh', width: '100%' }}>
      <GoogleMapReact
        bootstrapURLKeys={{ key: 'AIzaSyAnElPyLzdiSK-QFi9Lar05CZ_LwqHBFtI' }}
        defaultCenter={center}
        defaultZoom={10}
      >
        {Object.entries(locations).map(([userId, coordinates]) => (
          <Marker
            key={userId}
            lat={coordinates.lat}
            lng={coordinates.lng}
            text={userId}
          />
        ))}
      </GoogleMapReact>
    </div>
  );
}