Feature: Change color by manual input
	As a user
	I want to change the color by typing a hex code in the color input
	So that I can quickly see the preview of a different color

	Background:
		Given I see the preview generator page
		Then the brand input value is "Simple Icons"
		Then the color input value is "111111"

	Scenario: Change color by manual input
		When I type "FF4F64" in the color input
		Then the color in the preview is "Color: #FF4F64"
		Then the background color of the preview is #FF4F64
		Then the color of the badges in the preview is #FF4F64
