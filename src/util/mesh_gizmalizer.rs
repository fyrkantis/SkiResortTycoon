use bevy::{prelude::Color, gizmos::GizmoAsset};
use hexx::*;

#[allow(dead_code)] // TODO: Remove this function if still unused.
pub fn gizmalize(mesh_info: MeshInfo) -> GizmoAsset {
	let mut gizmo_asset = GizmoAsset::new();
	let (triangles, _) = mesh_info.indices.as_chunks::<3>();
	for [i1, i2, i3] in triangles {
		let (v1, v2, v3) = (mesh_info.vertices[*i1 as usize], mesh_info.vertices[*i2 as usize], mesh_info.vertices[*i3 as usize]);
		gizmo_asset.line(v1, v2, Color::srgb(1., 0., 0.));
		gizmo_asset.line(v2, v3, Color::srgb(0., 1., 0.));
		gizmo_asset.line(v3, v1, Color::srgb(0., 0., 1.));
	}
	gizmo_asset
}
