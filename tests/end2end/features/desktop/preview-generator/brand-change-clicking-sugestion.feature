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
		Then the brand input value is "Simple Analytics"
		Then the color input value is "FF4F64"
		Then the path input value starts with "M1.019 13.019"
		Then the title in the preview is "Simple Analytics Preview"
		Then the filename in the preview is "simpleanalytics.svg"
		Then the brand in the preview is "Brand: Simple Analytics"
		Then the color in the preview is "Color: #FF4F64"
		Then the background color of the preview is #FF4F64
		Then the SVG paths of the preview start with "M1.019 13.019"
