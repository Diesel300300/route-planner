// src/MapView.tsx
import  { useMemo } from 'react';
import Map, { Source, Layer } from 'react-map-gl/maplibre';
import 'maplibre-gl/dist/maplibre-gl.css';

// Example props: you’d pass in your array of nodes from Rust-WASM
type Node = { id: number; lat: number; lon: number };

export function MapView({ nodes }: { nodes: Node[] }) {
  // Optional extra safety:
  if (!nodes || nodes.length === 0) {
    return <div>Loading map data…</div>;
  }
  // 1) Build a GeoJSON FeatureCollection once
  const geojson = useMemo(() => ({
    type: 'FeatureCollection' as const,
    features: nodes.map(n => ({
      type: 'Feature' as const,
      geometry: {
        type: 'Point' as const,
        coordinates: [n.lon, n.lat],
      },
      properties: { id: n.id },
    })),
  }), [nodes]);

  // 2) Define your circle-layer style
  const layerStyle = useMemo(() => ({
    id: 'nodes-layer',
    type: 'circle' as const,
    paint: {
      'circle-radius': 10,
      'circle-color': '#007cbf',
      'circle-opacity': 1,
    },
  }), []);

  return (
    <Map
      initialViewState={{
        longitude: 0,
        latitude: 0,
        zoom: 2,
      }}
      style={{ width: '100vw', height: '100vh' }}
      mapStyle="https://api.maptiler.com/maps/basic-v2/style.json?key=4LPNKFS9ll0O8UdyvdYy"
    >
      <Source id="nodes" type="geojson" data={geojson}>
        <Layer {...layerStyle} />
      </Source>
    </Map>
  );
}

