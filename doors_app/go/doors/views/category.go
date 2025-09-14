package views

import (
	"fyne.io/fyne/v2"
	"fyne.io/fyne/v2/container"
	"fyne.io/fyne/v2/theme"
	"fyne.io/fyne/v2/widget"
)

type categoryItem struct {
	id       string
	parentId string
	title    string
	View     func(w fyne.Window) fyne.CanvasObject
}

type categoryItems struct {
	items []*categoryItem
}

func (c *categoryItems) ChildUIDs(uid string) (cids []string) {
	for _, item := range c.items {
		if item.parentId == uid {
			cids = append(cids, item.id)
		}
	}
	return
}

func (c *categoryItems) IsBranch(uid string) bool {
	for _, item := range c.items {
		if item.parentId == uid {
			return true
		}
	}
	return false
}

func (c *categoryItems) CreateNode(branch bool) fyne.CanvasObject {
	return widget.NewLabel("")
}

func (c *categoryItems) UpdateNode(uid string, branch bool, node fyne.CanvasObject) {
	for _, item := range c.items {
		if item.id == uid {
			node.(*widget.Label).SetText(item.title)
			return
		}
	}
}

func categoryView(content func(c fyne.CanvasObject)) fyne.CanvasObject {
	a := fyne.CurrentApp()
	items := &categoryItems{
		items: []*categoryItem{
			{id: "cat1", parentId: "", title: "Category 1"},
			{id: "item1", parentId: "", title: "Item 1", View: func(w fyne.Window) fyne.CanvasObject {
				return widget.NewLabel("This is Item 1")
			}},
			{id: "item2", parentId: "", title: "Item 2", View: func(w fyne.Window) fyne.CanvasObject {
				return widget.NewLabel("This is Item 2")
			}},
		},
	}

	tree := &widget.Tree{
		ChildUIDs:  items.ChildUIDs,
		IsBranch:   items.IsBranch,
		CreateNode: items.CreateNode,
		UpdateNode: items.UpdateNode,
		OnSelected: func(uid string) {
			if t, ok := tutorials.Tutorials[uid]; ok {
				for _, f := range tutorials.OnChangeFuncs {
					f()
				}
				tutorials.OnChangeFuncs = nil // Loading a page registers a new cleanup.

				a.Preferences().SetString(preferenceCurrentTutorial, uid)
				setTutorial(t)
			}
		}
	}
	themes := container.NewGridWithColumns(2,
		widget.NewButton("Dark", func() {
			a.Settings().SetTheme(&forcedVariant{Theme: theme.DefaultTheme(), variant: theme.VariantDark})
		}),
		widget.NewButton("Light", func() {
			a.Settings().SetTheme(&forcedVariant{Theme: theme.DefaultTheme(), variant: theme.VariantLight})
		}),
	)

	return container.NewBorder(nil, themes, nil, nil, tree)
}
