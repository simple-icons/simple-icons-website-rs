Feature: See all extensions in the extensions table
	As a user
	I want to see all available extensions in the extensions table
	So that I can easily find and install them

	Background:
		Given I see the index page
		When I click on the extensions button
		Then I see the extensions tables

	Scenario: The extensions table lists all available extensions
		Then I see an extension with the name "Blender add-on" and author "@mondeja"
		And I see an extension with the name "Figma plugin" and author "@LitoMore"
		And I see an extension with the name "Typst package" and author "@cscnk52"
		And I see an extension with the name "Kirby plugin" and author "@runxel"
		And I see an extension with the name "Python wheel" and author "@carstencodes"
