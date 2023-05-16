'use client';

import { use, useEffect, useState } from 'react';
import GoogleMapReact from 'google-map-react';
import 'tailwindcss/tailwind.css';

const Marker = ({ text }: any) => (
  <div className="group relative cursor-pointer">
    <div className="w-4 h-4 bg-blue-500 rounded-full transition duration-200 hover:bg-blue-700"></div>
    <p className="absolute bottom-full mb-2 text-xs text-white bg-gray-700 rounded px-2 opacity-0 group-hover:opacity-100 transition duration-200">
      {text}
    </p>
  </div>
);


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
  const [center, setCenter] = useState({ lat: 0, lng: 0 });
  const [zoom, setZoom] = useState(0);
  const ws = new WebSocket(LOCAL_URL + raceId);

  useEffect(() => {
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

      setCenter({ lat: json.coordinates.lat, lng: json.coordinates.long });
      setZoom(18);
    };

    return () => {
      ws.close();
      console.log("websocket connection closed");
    }
  }, [raceId, locations]);
  
  return (
    <div style={{ height: '100vh', width: '100%' }}>
      <GoogleMapReact
        bootstrapURLKeys={{ key: 'AIzaSyAnElPyLzdiSK-QFi9Lar05CZ_LwqHBFtI' }}
        defaultCenter={center}
        defaultZoom={zoom}
        center={center}
        zoom={zoom}
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