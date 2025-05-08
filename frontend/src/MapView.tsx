import { MapContainer, TileLayer, Marker, Popup } from "react-leaflet";
import "leaflet/dist/leaflet.css";

const DEFAULT_POSITION: [number, number] = [51.505, -0.09];

export function MapView() {
    return (
        <MapContainer
            center={DEFAULT_POSITION}
            zoom={13}
            className="h-80 w-full rounded-lg shadow-md"
        >
          <TileLayer
            url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
            attribution="&copy; OpenStreetMap contributors"
          />
          <Marker position={DEFAULT_POSITION}>
            <Popup>Here is London!</Popup>
          </Marker>
        </MapContainer>
    )
}
