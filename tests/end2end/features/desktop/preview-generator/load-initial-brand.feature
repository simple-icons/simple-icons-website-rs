Feature: Load default brand at start
	As a user
	I want to see the Simple Icons brand loaded by default
	So no external brands are promoted

	Background:
		Given I see the preview generator page
		Then the brand input value is "Simple Icons"

	Scenario: Load Simple Icons brand at start
		Then the color input value is "111111"
		Then the path input value starts with "M12 0"
		Then the title in the preview is "Simple Icons Preview"
		Then the filename in the preview is "simpleicons.svg"
		Then the brand in the preview is "Brand: Simple Icons"
		Then the color in the preview is "Color: #111111"
		Then the background color of the preview is #111111
		Then the SVG paths of the preview start with "M12 0"
