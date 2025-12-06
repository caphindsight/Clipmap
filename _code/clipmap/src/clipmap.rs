use godot::prelude::*;
use godot::classes::{
  PrimitiveMesh, IPrimitiveMesh,
  SurfaceTool,
  mesh::PrimitiveType,
};
use godot::global::{
  PropertyUsageFlags,
};
use godot::meta::{
  ClassId,
  PropertyInfo,
  PropertyHintInfo,
};

/// Class representing a clipmap mesh -
/// a plane with multiple configurable levels of detail
/// stitched together into a single mesh.
#[derive(GodotClass)]
#[class(tool, base=PrimitiveMesh, init)]
struct ClipmapMesh {
  base: Base<PrimitiveMesh>,
  lods: Vec<Lod>,
}

#[godot_api]
impl IPrimitiveMesh for ClipmapMesh {
  fn get_property_list(&mut self) -> Vec<PropertyInfo> {
    let mut res = Vec::new();
    res.push(PropertyInfo{
      variant_type: VariantType::INT,
      class_id: ClassId::none(),
      property_name: "clipmap/number_of_lods".into(),
      hint_info: PropertyHintInfo::none(),
      usage: PropertyUsageFlags::EDITOR | PropertyUsageFlags::STORAGE,
    });
    let n: usize = self.lods.len();
    for i in 0..n {
      res.push(PropertyInfo{
        variant_type: VariantType::FLOAT,
        class_id: ClassId::none(),
        property_name: (&(format!("clipmap/lod_{}/size", i))).into(),
        hint_info: PropertyHintInfo::none(),
        usage: PropertyUsageFlags::EDITOR | PropertyUsageFlags::STORAGE,
      });
      res.push(PropertyInfo{
        variant_type: VariantType::INT,
        class_id: ClassId::none(),
        property_name: (&(format!("clipmap/lod_{}/subdivisions", i))).into(),
        hint_info: PropertyHintInfo::none(),
        usage: PropertyUsageFlags::EDITOR | PropertyUsageFlags::STORAGE,
      });
    }
    res
  }

  fn get_property(&self, name: StringName) -> Option<Variant> {
    if streq(&name, "clipmap/number_of_lods") {
      return Some(Variant::from(self.lods.len() as i64));
    }
    if let Some(ind) = strcap1::<usize>("clipmap/lod_", &String::from_godot(GString::from(&name)), "/size") {
      if ind < self.lods.len() {
        return Some(Variant::from(self.lods[ind].size));
      }
      return None;
    }
    if let Some(ind) = strcap1::<usize>("clipmap/lod_", &String::from_godot(GString::from(&name)), "/subdivisions") {
      if ind < self.lods.len() {
        return Some(Variant::from(self.lods[ind].subd));
      }
      return None;
    }
    None
  }

  fn set_property(&mut self, name: StringName, value: Variant) -> bool {
    if streq(&name, "clipmap/number_of_lods") {
      if let Ok(val) = value.try_to::<u64>() {
        self.lods.resize(val as usize, Lod::default());
        self.update();
        return true;
      }
      return false;
    }
    if let Some(ind) = strcap1::<usize>("clipmap/lod_", &String::from_godot(GString::from(&name)), "/size") {
      if let Ok(val) = value.try_to::<f32>() {
        if ind >= self.lods.len() {
          self.lods.resize(ind + 1, Lod::default());
        }
        self.lods[ind].size = val;
        self.update();
        return true;
      }
      return false;
    }
    if let Some(ind) = strcap1::<usize>("clipmap/lod_", &String::from_godot(GString::from(&name)), "/subdivisions") {
      if let Ok(val) = value.try_to::<i32>() {
        if ind >= self.lods.len() {
          self.lods.resize(ind + 1, Lod::default());
        }
        self.lods[ind].subd = val;
        self.update();
        return true;
      }
      return false;
    }
    false
  }

  fn create_mesh_array(&self) -> Array<Variant> {
    let n_lods: usize = self.lods.len();

    let mut st_gd = SurfaceTool::new_gd();
    let st: &mut SurfaceTool = &mut st_gd;
    st.begin(PrimitiveType::TRIANGLES);
    st.set_normal(Vector3::new(0., 1., 0.));

    if n_lods == 0 {
      add_tri(st, Vector2::new(0., 0.), Vector2::new(0., 0.), Vector2::new(0., 0.));
      return st.commit_to_arrays();
    }

    let mut lod_idx: Vec<usize> = Vec::new();
    lod_idx.resize(n_lods, 0);
    for i in 0 .. n_lods {
      lod_idx[i] = i;
    }
    lod_idx.sort_by(|a: &usize, b: &usize| {
      if self.lods[*a].size < self.lods[*b].size {
        return std::cmp::Ordering::Less;
      }
      if self.lods[*a].size > self.lods[*b].size {
        return std::cmp::Ordering::Greater;
      }
      std::cmp::Ordering::Equal
    });

    for l in 0 .. n_lods {
      let lod = lod_idx[l];
      let quad_side: f32 = self.lods[lod].size / ((self.lods[lod].subd + 1) as f32);
      let offset = Vector2::new(-self.lods[lod].size / 2., -self.lods[lod].size / 2.);
      let has_hole: bool = l > 0;

      let hole_size: f32 = if has_hole { self.lods[lod_idx[l - 1]].size } else { 0. };
      let hole_subd: i32 = if has_hole { self.lods[lod_idx[l - 1]].subd } else { 0 };
      let hole_side: f32 = hole_size / ((hole_subd + 1) as f32);

      let has_hole: bool = has_hole && hole_size > 0.;

      let is_in_hole_fn = |p: Vector2| {
        has_hole && p.x.abs() <= hole_size / 2. && p.y.abs() <= hole_size / 2.
      };

      for i in 0 .. (self.lods[lod].subd + 1) {
        for j in 0 .. (self.lods[lod].subd + 1) {
          let a = offset + Vector2::new(quad_side * (i as f32), quad_side * (j as f32));
          let b = a + Vector2::new(quad_side, 0.);
          let c = b + Vector2::new(0., quad_side);
          let d = a + Vector2::new(0., quad_side);
          if !is_in_hole_fn(a) && !is_in_hole_fn(b) && !is_in_hole_fn(c) && !is_in_hole_fn(d) {
            add_quad(st, a, b, c, d, (i + j) % 2 == 1);
          }
        }
      }

      if has_hole {
        let mut i1: i32 = 0;
        while i1 <= self.lods[lod].subd && offset.x + ((i1 + 1) as f32) * quad_side < -hole_size / 2. {
          i1 += 1;
        }
        let i1: i32 = i1;
        let i2: i32 = self.lods[lod].subd + 1 - i1;

        let f1 = i1 as f32;
        let f2 = i2 as f32;

        // Stitch the top seam.
        stitch(st,
          offset + Vector2::new(f1 * quad_side, f1 * quad_side),
          quad_side,
          i2 - i1,
          Vector2::new(-hole_size / 2., -hole_size / 2.),
          hole_side,
          hole_subd + 1,
          Vector2::new(1., 0.)
        );

        // Stitch the right seam.
        stitch(st,
          offset + Vector2::new(f2 * quad_side, f1 * quad_side),
          quad_side,
          i2 - i1,
          Vector2::new(hole_size / 2., -hole_size / 2.),
          hole_side,
          hole_subd + 1,
          Vector2::new(0., 1.)
        );

        // Stitch the bottom seam.
        stitch(st,
          offset + Vector2::new(f2 * quad_side, f2 * quad_side),
          quad_side,
          i2 - i1,
          Vector2::new(hole_size / 2., hole_size / 2.),
          hole_side,
          hole_subd + 1,
          Vector2::new(-1., 0.)
        );

        // Stitch the left seam.
        stitch(st,
          offset + Vector2::new(f1 * quad_side, f2 * quad_side),
          quad_side,
          i2 - i1,
          Vector2::new(-hole_size / 2., hole_size / 2.),
          hole_side,
          hole_subd + 1,
          Vector2::new(0., -1.)
        );
      }
    }

    st.commit_to_arrays()
  }
}

impl ClipmapMesh {
  fn update(&mut self) {
    self.base_mut().notify_property_list_changed();
    self.base_mut().request_update();
  }
}

#[derive(Clone, Copy, Default)]
struct Lod {
  size: f32,
  subd: i32,
}

fn streq(str1: &StringName, str2: &str) -> bool {
  str1.chars().iter().copied().eq(str2.chars())
}

fn strcap1<T>(prefix: &str, string: &str, suffix: &str) -> Option<T>
where T: Copy + std::str::FromStr + Sized {
  if string.len() >= prefix.len() + suffix.len() && string.starts_with(prefix) && string.ends_with(suffix) {
    let subs = &string[prefix.len() .. string.len() - suffix.len()];
    if let Ok(x) = subs.parse::<T>() {
      return Some(x);
    }
  }
  None
}

fn add_tri(st: &mut SurfaceTool, a: Vector2, b: Vector2, c: Vector2) {
  st.add_vertex(Vector3::new(a.x, 0., a.y));
  st.add_vertex(Vector3::new(b.x, 0., b.y));
  st.add_vertex(Vector3::new(c.x, 0., c.y));
}

fn add_quad(st: &mut SurfaceTool, a: Vector2, b: Vector2, c: Vector2, d: Vector2, checkerboard: bool) {
  if checkerboard {
    add_tri(st, a, b, d);
    add_tri(st, b, c, d);
  } else {
    add_tri(st, a, b, c);
    add_tri(st, a, c, d);
  }
}

fn stitch(
  st: &mut SurfaceTool,
  a_start: Vector2, da: f32, na: i32,
  b_start: Vector2, db: f32, nb: i32,
  dir: Vector2
) {
  let mut i: i32 = 1;
  let mut j: i32 = 1;
  while i <= na || j <= nb {
    if j > nb || dir.dot(a_start + dir * da * (i as f32)) <= dir.dot(b_start + dir * db * (j as f32)) {
      add_tri(st,
        a_start + dir * da * ((i - 1) as f32),
        a_start + dir * da * (i as f32),
        b_start + dir * db * ((j - 1) as f32)
      );
      i += 1;
    } else {
      add_tri(st,
        a_start + dir * da * ((i - 1) as f32),
        b_start + dir * db * (j as f32),
        b_start + dir * db * ((j - 1) as f32)
      );
      j += 1;
    }
  }
}
