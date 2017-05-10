#ifndef CANADA_H_
#define CANADA_H_

#include <map>
#include <vector>
#include <experimental/optional>

#include "rapidjson/reader.h"

enum ObjType {
  kFeatureCollectionObj,
  kFeatureObj,
  kPolygonObj,
};

struct Coordinate {
  float latitude_;
  float longitude_;
};

struct Geometry {
  ObjType type_;
  std::vector<std::vector<Coordinate>> coordinates_;
};

struct Feature {
  ObjType type_;
  std::map<std::string, std::string> properties_;
  Geometry geometry_;
};

struct FeatureCollection {
  ObjType type_;
  std::vector<Feature> features_;
};

template <typename Writer>
void Serialize(const ObjType& self, Writer& writer);

template <typename Writer>
void Serialize(const Coordinate& self, Writer& writer);

template <typename Writer>
void Serialize(const Geometry& self, Writer& writer);

template <typename Writer>
void Serialize(const Feature& self, Writer& writer);

template <typename Writer>
void Serialize(const FeatureCollection& self, Writer& writer);

struct Canada : public rapidjson::BaseReaderHandler<rapidjson::UTF8<>, Canada> {
  Canada();
  FeatureCollection Get();
  bool StartObject();
  bool EndObject(rapidjson::SizeType);
  bool StartArray();
  bool EndArray(rapidjson::SizeType);
  bool Key(const char* str, rapidjson::SizeType len, bool);
  bool String(const char* str, rapidjson::SizeType len, bool);
  bool Double(double d);
  bool Int(int i);
  bool Uint(unsigned u);
  bool Default();

  enum State {
    kFeatureCollection,
    kFeatureCollectionKey,
    kFeatureCollectionType,
    kFeatureCollectionFeatures,
    kFeatureCollectionEnd,
    kFeature,
    kFeatureKey,
    kFeatureType,
    kFeatureProperties,
    kFeatureGeometry,
    kPropertyKey,
    kPropertyValue,
    kGeometryKey,
    kGeometryType,
    kGeometryCoordinates,
    kCoordinateArrayArray,
    kCoordinateArray,
    kCoordinate,
    kCoordinateLatitude,
    kCoordinateLongitude,
    kCoordinateEnd,
  } state_;
  std::experimental::optional<ObjType> feature_collection_type_;
  std::experimental::optional<ObjType> feature_type_;
  std::experimental::optional<ObjType> geometry_type_;
  std::experimental::optional<std::vector<Feature>> features_;
  std::experimental::optional<std::string> property_;
  std::experimental::optional<std::map<std::string, std::string>> properties_;
  std::experimental::optional<Geometry> geometry_;
  std::experimental::optional<std::vector<std::vector<Coordinate>>> coordinates_;
  std::experimental::optional<float> latitude_;
};

#endif // CANADA_H_
