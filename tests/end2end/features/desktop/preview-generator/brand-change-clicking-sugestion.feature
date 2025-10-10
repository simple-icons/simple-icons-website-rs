Feature: Change brand by minimal suggestion
	As a user
	I want to change the brand by clicking on the first suggestion
	So that I can quickly see the preview of a different brand

	Background:
		Given I see the preview generator page
		Then The brand input value is "Simple Icons"
		Then The color input value is "111111"
		Then The path input value starts with "M12 0"
		Then The title in the preview is "Simple Icons Preview"
		Then The filename in the preview is "simpleicons.svg"
		Then The brand in the preview is "Brand: Simple Icons"
		Then The color in the preview is "Color: #111111"
		Then The background color of the preview is #111111
		Then The SVG paths of the preview start with "M12 0"

	Scenario: Change brand by minimal suggestion
		When I focus on the brand input
		Then I can see some brand suggestions
		When I click on the first brand suggestion
		Then The brand input value is "Simple Analytics"
		Then The color input value is "FF4F64"
		Then The path input value starts with "M1.019 13.019"
		Then The title in the preview is "Simple Analytics Preview"
		Then The filename in the preview is "simpleanalytics.svg"
		Then The brand in the preview is "Brand: Simple Analytics"
		Then The color in the preview is "Color: #FF4F64"
		Then The background color of the preview is #FF4F64
		Then The SVG paths of the preview start with "M1.019 13.019"
