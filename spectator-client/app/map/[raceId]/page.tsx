'use client';

import { useEffect, useState } from 'react';
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

interface UserLocations {
    [userId: string]: LocationUpdate[];
}

interface LocationUpdate {
    userId: string;
    timestamp: number;
    coordinates: {
        lat: number;
        long: number;
    };
}



export default function MapPage({params}: any) {
  const isLocal = typeof window === 'undefined' || window.location.hostname === 'localhost';
  const REMOTE_URL = 'wss://websockets.fly.dev/race/';
  const LOCAL_URL  = 'ws://localhost:8080/race/';

const url = isLocal ? LOCAL_URL : REMOTE_URL;

  const raceId = params.raceId
  const [locations, setLocations] = useState<UserLocations>(Object.create(null));
  const [center, setCenter] = useState({ lat: 0, lng: 0 });
  const [zoom, setZoom] = useState(0);

  useEffect(() => {
    const ws = new WebSocket(url + raceId);

    ws.onopen = () => {  
      console.log("websocket connection established");
    };

    ws.onmessage = (event: any) => {
      const json = JSON.parse(event.data);
      console.log(json);
      setLocations(prevLocations => {
        const userLocations = prevLocations[json.userId] || [];
        const updatedUserLocations = [...userLocations, json];
        return { 
          ...prevLocations, 
          [json.userId]: updatedUserLocations
        };
      });
    };

    return () => {
      ws.close();
      console.log("websocket connection closed");
    }
  }, [raceId]);

  return (
    <div style={{ height: '100vh', width: '100%' }}>
      <GoogleMapReact
        bootstrapURLKeys={{ key: 'AIzaSyAmG6gfiY8QojDxjQSes-7S_xLQmJjjUcQ' }}
        defaultCenter={center}
        defaultZoom={zoom}
        center={center}
        zoom={zoom}
      >
        {Object.entries(locations).map(([userId, userLocations]) => {
          const lastLocation = userLocations[userLocations.length - 1].coordinates;
          return (
            <Marker
              key={userId}
              lat={lastLocation.lat}
              lng={lastLocation.long}
              text={userId}
            />
          )
        })}
      </GoogleMapReact>
    </div>
  );
}
