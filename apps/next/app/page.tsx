'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import 'tailwindcss/tailwind.css';

export default function Home() {
  const router = useRouter();
  const [raceId, setRaceId] = useState('');

  const handleSubmit = (event: { preventDefault: () => void; }) => {
    event.preventDefault();
    router.push(`/map/${raceId}`);
  };

  return (
    <div className="flex items-center justify-center h-screen bg-gray-800">
      <form onSubmit={handleSubmit} className="flex flex-col items-center">
        <h1 className="mb-4 text-4xl text-white">Spectate real-time endurance race</h1>
        <input
          type="text"
          placeholder="Enter Race ID"
          value={raceId}
          onChange={(e) => setRaceId(e.target.value)}
          className="p-2 text-white bg-gray-700 rounded"
        />
        <button type="submit" className="p-2 mt-2 text-white bg-blue-500 rounded">
          Submit
        </button>
      </form>
    </div>
  );
}