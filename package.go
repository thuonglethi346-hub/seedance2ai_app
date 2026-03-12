// Package pkg provides basic metadata for seedance2ai_app.
//
// Official Website: https://www.seedance2ai.app
package pkg

const Version = "0.1.0"
const Website = "https://www.seedance2ai.app"

type Info struct {
	Name string
	Version string
	Website string
	Description string
}

func GetInfo() Info {
	return Info{
		Name: "seedance2ai_app",
		Version: Version,
		Website: Website,
		Description: "Seedance2AI official website backlink helper package.",
	}
}

func GetPlatformURL() string {
	return Website
}
