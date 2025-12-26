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

	Scenario: Change color by manual input with leading "#"
		When I type "#123123" in the color input
		Then the color in the preview is "Color: #123123"
		Then the background color of the preview is #123123
		Then the color of the badges in the preview is #123123

	Scenario: Change color by manual input with lowercase letters
		When I type "ab12cd" in the color input
		Then the color in the preview is "Color: #AB12CD"
		Then the background color of the preview is #AB12CD
		Then the color of the badges in the preview is #AB12CD

	Scenario: Change color by manual input with mixed case letters
		When I type "aB12Cd" in the color input
		Then the color in the preview is "Color: #AB12CD"
		Then the background color of the preview is #AB12CD
		Then the color of the badges in the preview is #AB12CD

	Scenario: Change color by manual input with lowercase letters and leading "#"
		When I type "#ab12cd" in the color input
		Then the color in the preview is "Color: #AB12CD"
		Then the background color of the preview is #AB12CD
		Then the color of the badges in the preview is #AB12CD
