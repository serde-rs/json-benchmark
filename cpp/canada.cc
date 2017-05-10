#include "rapidjson/writer.h"

#include "canada.h"
#include "util.h"

// ObjType variants
const char kFeatureCollection[] = "FeatureCollection";
const char kFeature[] = "Feature";
const char kPolygon[] = "Polygon";

// Fields
const char kType[] = "type";
const char kFeatures[] = "features";
const char kProperties[] = "properties";
const char kGeometry[] = "geometry";
const char kCoordinates[] = "coordinates";

template <typename Writer>
void Serialize(const ObjType& self, Writer& writer) {
  switch (self) {
    case kFeatureCollectionObj:
      writer.String(kFeatureCollection);
      break;
    case kFeatureObj:
      writer.String(kFeature);
      break;
    case kPolygonObj:
      writer.String(kPolygon);
      break;
  }
}

template <typename Writer>
void Serialize(const Coordinate& self, Writer& writer) {
  writer.StartArray();
  writer.Double(self.latitude_);
  writer.Double(self.longitude_);
  writer.EndArray();
}

template <typename Writer>
void Serialize(const Geometry& self, Writer& writer) {
  writer.StartObject();

  writer.String(kType);
  Serialize(self.type_, writer);

  writer.String(kCoordinates);
  writer.StartArray();
  for (auto&& coordinate_array : self.coordinates_) {
    writer.StartArray();
    for (auto&& coordinate : coordinate_array) {
      Serialize(coordinate, writer);
    }
    writer.EndArray();
  }
  writer.EndArray();

  writer.EndObject();
}

template <typename Writer>
void Serialize(const Feature& self, Writer& writer) {
  writer.StartObject();

  writer.String(kType);
  Serialize(self.type_, writer);

  writer.String(kProperties);
  writer.StartObject();
  for (auto&& property : self.properties_) {
    writer.String(property.first.c_str());
    writer.String(property.second.c_str());
  }
  writer.EndObject();

  writer.String(kGeometry);
  Serialize(self.geometry_, writer);

  writer.EndObject();
}

template <typename Writer>
void Serialize(const FeatureCollection& self, Writer& writer) {
  writer.StartObject();

  writer.String(kType);
  Serialize(self.type_, writer);

  writer.String(kFeatures);
  writer.StartArray();
  for (auto&& feature : self.features_) {
    Serialize(feature, writer);
  }
  writer.EndArray();

  writer.EndObject();
}

using Writer = rapidjson::Writer<rapidjson::StringBuffer>;
template void Serialize<Writer>(const ObjType& self, Writer& writer);
template void Serialize<Writer>(const Coordinate& self, Writer& writer);
template void Serialize<Writer>(const Geometry& self, Writer& writer);
template void Serialize<Writer>(const Feature& self, Writer& writer);
template void Serialize<Writer>(const FeatureCollection& self, Writer& writer);

bool ParseObjType(const char* str, rapidjson::SizeType len, ObjType& out) {
  if (Eq(str, len, kFeatureCollection, sizeof(kFeatureCollection) - 1)) {
    out = kFeatureCollectionObj;
    return true;
  } else if (Eq(str, len, kFeature, sizeof(kFeature) - 1)) {
    out = kFeatureObj;
    return true;
  } else if (Eq(str, len, kPolygon, sizeof(kPolygon) - 1)) {
    out = kPolygonObj;
    return true;
  } else {
    return false;
  }
}

Canada::Canada()
  : state_(kFeatureCollection),
    feature_collection_type_(),
    feature_type_(),
    geometry_type_(),
    features_(),
    property_(),
    properties_(),
    geometry_(),
    coordinates_(),
    latitude_()
{}

FeatureCollection Canada::Get() {
  return FeatureCollection {
    .type_ = *feature_collection_type_,
    .features_ = *features_,
  };
}

bool Canada::StartObject() {
  switch (state_) {
    case kFeatureCollection:
      state_ = kFeatureCollectionKey;
      return true;
    case kFeature:
      state_ = kFeatureKey;
      return true;
    case kFeatureProperties:
      properties_.emplace();
      state_ = kPropertyKey;
      return true;
    case kFeatureGeometry:
      state_ = kGeometryKey;
      return true;
    default:
      return false;
  }
}

bool Canada::EndObject(rapidjson::SizeType) {
  switch (state_) {
    case kFeatureCollectionKey:
      if (!feature_collection_type_ || !features_) {
        return false;
      }
      state_ = kFeatureCollectionEnd;
      return true;
    case kFeatureKey:
      if (!feature_type_ || !properties_ || !geometry_) {
        return false;
      }
      features_->push_back(Feature {
        .type_ = *feature_type_,
        .properties_ = *properties_,
        .geometry_ = *geometry_,
      });
      feature_type_ = std::experimental::nullopt;
      properties_ = std::experimental::nullopt;
      geometry_ = std::experimental::nullopt;
      state_ = kFeature;
      return true;
    case kPropertyKey:
      state_ = kFeatureKey;
      return true;
    case kGeometryKey:
      if (!geometry_type_ || !coordinates_) {
        return false;
      }
      geometry_ = Geometry {
        .type_ = *geometry_type_,
        .coordinates_ = *coordinates_,
      };
      geometry_type_ = std::experimental::nullopt;
      coordinates_ = std::experimental::nullopt;
      state_ = kFeatureKey;
      return true;
    default:
      return false;
  }
}

bool Canada::StartArray() {
  switch (state_) {
    case kFeatureCollectionFeatures:
      features_.emplace();
      state_ = kFeature;
      return true;
    case kCoordinateArrayArray:
      coordinates_.emplace();
      state_ = kCoordinateArray;
      return true;
    case kCoordinateArray:
      coordinates_->emplace_back();
      state_ = kCoordinate;
      return true;
    case kCoordinate:
      state_ = kCoordinateLatitude;
      return true;
    default:
      return false;
  }
}

bool Canada::EndArray(rapidjson::SizeType) {
  switch (state_) {
    case kFeature:
      state_ = kFeatureCollectionKey;
      return true;
    case kCoordinateEnd:
      state_ = kCoordinate;
      return true;
    case kCoordinate:
      state_ = kCoordinateArray;
      return true;
    case kCoordinateArray:
      state_ = kGeometryKey;
      return true;
    default:
      return false;
  }
}

bool Canada::Key(const char* str, rapidjson::SizeType len, bool) {
  switch (state_) {
    case kFeatureCollectionKey:
      if (Eq(str, len, kType, sizeof(kType) - 1)) {
        if (feature_collection_type_) {
          return false;
        }
        state_ = kFeatureCollectionType;
        return true;
      } else if (Eq(str, len, kFeatures, sizeof(kFeatures) - 1)) {
        if (features_) {
          return false;
        }
        state_ = kFeatureCollectionFeatures;
        return true;
      } else {
        return false;
      }
    case kFeatureKey:
      if (Eq(str, len, kType, sizeof(kType) - 1)) {
        if (feature_type_) {
          return false;
        }
        state_ = kFeatureType;
        return true;
      } else if (Eq(str, len, kProperties, sizeof(kProperties) - 1)) {
        if (properties_) {
          return false;
        }
        state_ = kFeatureProperties;
        return true;
      } else if (Eq(str, len, kGeometry, sizeof(kGeometry) - 1)) {
        if (geometry_) {
          return false;
        }
        state_ = kFeatureGeometry;
        return true;
      } else {
        return false;
      }
    case kPropertyKey:
      property_ = std::string(str, len);
      state_ = kPropertyValue;
      return true;
    case kGeometryKey:
      if (Eq(str, len, kType, sizeof(kType) - 1)) {
        if (geometry_type_) {
          return false;
        }
        state_ = kGeometryType;
        return true;
      } else if (Eq(str, len, kCoordinates, sizeof(kCoordinates) - 1)) {
        if (coordinates_) {
          return false;
        }
        state_ = kCoordinateArrayArray;
        return true;
      } else {
        return false;
      }
    default:
      return false;
  }
}

bool Canada::String(const char* str, rapidjson::SizeType len, bool) {
  switch (state_) {
    case kFeatureCollectionType:
      ObjType feature_collection_type;
      if (!ParseObjType(str, len, feature_collection_type)) {
        return false;
      }
      feature_collection_type_ = feature_collection_type;
      state_ = kFeatureCollectionKey;
      return true;
    case kFeatureType:
      ObjType feature_type;
      if (!ParseObjType(str, len, feature_type)) {
        return false;
      }
      feature_type_ = feature_type;
      state_ = kFeatureKey;
      return true;
    case kPropertyValue:
      properties_->insert(std::make_pair(*property_, std::string(str, len)));
      property_ = std::experimental::nullopt;
      state_ = kPropertyKey;
      return true;
    case kGeometryType:
      ObjType geometry_type;
      if (!ParseObjType(str, len, geometry_type)) {
        return false;
      }
      geometry_type_ = geometry_type;
      state_ = kGeometryKey;
      return true;
    default:
      return false;
  }
}

bool Canada::Double(double d) {
  switch (state_) {
    case kCoordinateLatitude:
      latitude_ = d;
      state_ = kCoordinateLongitude;
      return true;
    case kCoordinateLongitude:
      coordinates_->back().push_back(Coordinate {
        .latitude_ = *latitude_,
        .longitude_ = float(d),
      });
      latitude_ = std::experimental::nullopt;
      state_ = kCoordinateEnd;
      return true;
    default:
      return false;
  }
}

bool Canada::Int(int i) {
  return Double(i);
}

bool Canada::Uint(unsigned u) {
  return Double(u);
}

bool Canada::Default() {
  // All other events are invalid.
  return false;
}
