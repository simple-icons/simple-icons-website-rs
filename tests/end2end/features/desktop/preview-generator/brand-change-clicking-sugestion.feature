Feature: Change brand by minimal suggestion
	As a user
	I want to change the brand by clicking on the first suggestion
	So that I can quickly see the preview of a different brand

	Background:
		Given I see the preview generator page
		Then the brand input value is "Simple Icons"

	Scenario: Change brand by minimal suggestion
		When I click the brand input
		Then I can see some brand suggestions
		When I click on the first brand suggestion
		Then the brand input value is "Simple Icons CDN"
		Then the color input value is "111111"
		Then the path input value starts with "M20.11656 23.52021"
		Then the title in the preview is "Simple Icons CDN Preview"
		Then the filename in the preview is "simpleiconscdn.svg"
		Then the brand in the preview is "Brand: Simple Icons CDN"
		Then the color in the preview is "Color: #111111"
		Then the background color of the preview is #111111
		Then the SVG paths of the preview start with "M20.11656 23.52021"
