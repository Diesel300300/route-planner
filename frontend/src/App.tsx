import { MapView } from './MapView'

function App() {
    const nodes = [
        { id: 1, lat: 51.068, lon: 4.03 },
        { id: 2, lat: 51.069, lon: 4.03 }
    ];
  return (
    <div className='h-screen p-4 space-y-4 bg-gray-200'>
      <h1 className="text-2xl font-semibold">My Map Application</h1>
      <MapView nodes={nodes}/>
    </div>
  ) 
}

export default App
