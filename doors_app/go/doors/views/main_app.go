package views

import (
	"fyne.io/fyne/v2"
	"fyne.io/fyne/v2/app"
	"fyne.io/fyne/v2/container"
)

func MainApp() {
	a := app.New()
	w := a.NewWindow("Doors")
	w.Resize(fyne.NewSize(800, 600))
	w.CenterOnScreen()

	content := container.NewStack()
	setContent := func(c fyne.CanvasObject) {
		content.Objects = []fyne.CanvasObject{c}
		content.Refresh()
	}
	{
		split := container.NewHSplit(categoryView(setContent), content)
		split.Offset = 0.2
		w.SetContent(split)
	}
	w.ShowAndRun()
}