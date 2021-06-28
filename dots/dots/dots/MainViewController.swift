//
//  MainViewController.swift
//  dots
//
//  Created by Brian Tiger Chow on 6/27/21.
//

import Foundation
import UIKit
import DotsLib

class MainViewController: UICollectionViewController {

    var dataSource: UICollectionViewDiffableDataSource<Section, Item>?

    let store: Store

    init(store: Store) {
        self.store = store
        let layout = UICollectionViewFlowLayout()
        super.init(collectionViewLayout: layout)
        collectionView.register(Cell.self, forCellWithReuseIdentifier: Cell.reuseID)
    }

    required init?(coder: NSCoder) {
        fatalError()
    }

    override func viewDidLoad() {
        dataSource = UICollectionViewDiffableDataSource<Section, Item>(collectionView: collectionView) {
            a, indexPath, c in
            switch c {
            case .row:
                // has buttons
            }
            return a.dequeueReusableCell(withReuseIdentifier: Cell.reuseID, for: indexPath)
        }
    }
}

enum Section: Hashable {
    case heading
    case body
}

enum Item: Hashable {
    case row
}

class Cell: UICollectionViewCell {
    static let reuseID = "Cell"
}
